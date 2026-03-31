use super::{Actor, ActorId, ActorStatus, Item, ItemId, ItemStatus};

pub const MAX_FINE_CENTS: i64 = 5_000;

pub fn calculate_overdue_fine(overdue_days: u32, daily_rate_cents: i64) -> i64 {
    let raw = (overdue_days as i64).saturating_mul(daily_rate_cents);
    raw.min(MAX_FINE_CENTS)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckoutCommand {
    pub item_id: ItemId,
    pub patron_id: ActorId,
    pub timestamp: i64,
    pub correlation_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CirculationEvent {
    ItemCheckedOut {
        item_id: ItemId,
        patron_id: ActorId,
        timestamp: i64,
        correlation_id: String,
    },
    ItemReturned {
        item_id: ItemId,
        patron_id: ActorId,
        timestamp: i64,
        correlation_id: String,
    },
    ItemRenewed {
        item_id: ItemId,
        patron_id: ActorId,
        timestamp: i64,
        correlation_id: String,
    },
    HoldPlaced {
        item_id: ItemId,
        patron_id: ActorId,
        timestamp: i64,
        correlation_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckoutError {
    ItemNotAvailable,
    ItemMismatch,
    PatronMismatch,
    PatronBlocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemAggregate {
    pub item_id: Option<ItemId>,
    pub status: ItemStatus,
    pub checked_out_to: Option<ActorId>,
    pub last_timestamp: Option<i64>,
    pub last_correlation_id: Option<String>,
}

impl ItemAggregate {
    pub fn process(
        item: &Item,
        patron: &Actor,
        command: CheckoutCommand,
    ) -> Result<Vec<CirculationEvent>, CheckoutError> {
        if item.id != command.item_id {
            return Err(CheckoutError::ItemMismatch);
        }

        if patron.id != command.patron_id {
            return Err(CheckoutError::PatronMismatch);
        }

        if patron.status != ActorStatus::Active {
            return Err(CheckoutError::PatronBlocked);
        }

        if item.status != ItemStatus::Available {
            return Err(CheckoutError::ItemNotAvailable);
        }

        Ok(vec![CirculationEvent::ItemCheckedOut {
            item_id: command.item_id,
            patron_id: command.patron_id,
            timestamp: command.timestamp,
            correlation_id: command.correlation_id,
        }])
    }

    pub fn rebuild(events: &[CirculationEvent]) -> Self {
        let mut aggregate = Self {
            item_id: None,
            status: ItemStatus::Available,
            checked_out_to: None,
            last_timestamp: None,
            last_correlation_id: None,
        };

        for event in events {
            match event {
                CirculationEvent::ItemCheckedOut {
                    item_id,
                    patron_id,
                    timestamp,
                    correlation_id,
                }
                | CirculationEvent::ItemRenewed {
                    item_id,
                    patron_id,
                    timestamp,
                    correlation_id,
                }
                | CirculationEvent::HoldPlaced {
                    item_id,
                    patron_id,
                    timestamp,
                    correlation_id,
                } => {
                    aggregate.item_id = Some(item_id.clone());
                    aggregate.checked_out_to = Some(patron_id.clone());
                    aggregate.status = match event {
                        CirculationEvent::ItemCheckedOut { .. } => ItemStatus::CheckedOut,
                        CirculationEvent::ItemRenewed { .. } => ItemStatus::CheckedOut,
                        CirculationEvent::HoldPlaced { .. } => ItemStatus::OnHoldShelf,
                        CirculationEvent::ItemReturned { .. } => ItemStatus::Available,
                    };
                    aggregate.last_timestamp = Some(*timestamp);
                    aggregate.last_correlation_id = Some(correlation_id.clone());
                }
                CirculationEvent::ItemReturned {
                    item_id,
                    patron_id: _,
                    timestamp,
                    correlation_id,
                } => {
                    aggregate.item_id = Some(item_id.clone());
                    aggregate.status = ItemStatus::Available;
                    aggregate.checked_out_to = None;
                    aggregate.last_timestamp = Some(*timestamp);
                    aggregate.last_correlation_id = Some(correlation_id.clone());
                }
            }
        }

        aggregate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Actor, ActorId, ActorStatus, ActorType, BranchId, Item, ItemId};
    use proptest::prelude::*;

    impl Arbitrary for ItemId {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            "[a-z0-9_-]{1,16}"
                .prop_map(|value| ItemId(value.to_string()))
                .boxed()
        }
    }

    impl Arbitrary for ActorId {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            "[a-z0-9_-]{1,16}"
                .prop_map(|value| ActorId(value.to_string()))
                .boxed()
        }
    }

    impl Arbitrary for BranchId {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            "[a-z0-9_-]{1,16}"
                .prop_map(|value| BranchId(value.to_string()))
                .boxed()
        }
    }

    impl Arbitrary for ActorType {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(ActorType::Patron),
                Just(ActorType::Librarian),
                Just(ActorType::Admin),
                Just(ActorType::Machine),
            ]
            .boxed()
        }
    }

    impl Arbitrary for ActorStatus {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(ActorStatus::Active),
                Just(ActorStatus::Suspended),
                Just(ActorStatus::Blocked),
                Just(ActorStatus::Expired),
            ]
            .boxed()
        }
    }

    impl Arbitrary for ItemStatus {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(ItemStatus::Available),
                Just(ItemStatus::CheckedOut),
                Just(ItemStatus::OnHoldShelf),
                Just(ItemStatus::Lost),
                Just(ItemStatus::InTransit),
            ]
            .boxed()
        }
    }

    impl Arbitrary for Item {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (
                any::<ItemId>(),
                "[a-z0-9_-]{1,16}".prop_map(|value| value.to_string()),
                "[a-z0-9_-]{1,16}".prop_map(|value| value.to_string()),
                any::<ItemStatus>(),
                any::<BranchId>(),
            )
                .prop_map(|(id, bib_id, barcode, status, location)| Item {
                    id,
                    bib_id,
                    barcode,
                    status,
                    location,
                })
                .boxed()
        }
    }

    impl Arbitrary for Actor {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (
                any::<ActorId>(),
                any::<ActorType>(),
                "[a-zA-Z0-9_-]{1,16}".prop_map(|value| value.to_string()),
                prop_oneof![
                    Just(None),
                    "[a-zA-Z0-9._%+-]{1,24}@[a-zA-Z0-9.-]{1,24}".prop_map(Some)
                ],
                any::<ActorStatus>(),
            )
                .prop_map(|(id, actor_type, name, email, status)| Actor {
                    id,
                    actor_type,
                    name,
                    email,
                    status,
                })
                .boxed()
        }
    }

    impl Arbitrary for CheckoutCommand {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (
                any::<ItemId>(),
                any::<ActorId>(),
                any::<i64>(),
                "[a-z0-9_-]{1,24}".prop_map(|value| value.to_string()),
            )
                .prop_map(
                    |(item_id, patron_id, timestamp, correlation_id)| CheckoutCommand {
                        item_id,
                        patron_id,
                        timestamp,
                        correlation_id,
                    },
                )
                .boxed()
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn available_item_checkout_succeeds(
            mut item in any::<Item>(),
            mut patron in any::<Actor>(),
            timestamp in any::<i64>(),
            correlation_id in "[a-z0-9_-]{1,24}".prop_map(|value| value.to_string()),
        ) {
            item.status = ItemStatus::Available;
            patron.actor_type = ActorType::Patron;
            patron.status = ActorStatus::Active;

            let command = CheckoutCommand {
                item_id: item.id.clone(),
                patron_id: patron.id.clone(),
                timestamp,
                correlation_id,
            };

            let result = ItemAggregate::process(&item, &patron, command);

            let success = matches!(
                result,
                Ok(ref events) if matches!(&events[..], [CirculationEvent::ItemCheckedOut { .. }])
            );

            prop_assert!(success);
        }

        #[test]
        fn blocked_patron_checkout_fails(
            mut item in any::<Item>(),
            mut patron in any::<Actor>(),
            timestamp in any::<i64>(),
            correlation_id in "[a-z0-9_-]{1,24}".prop_map(|value| value.to_string()),
        ) {
            item.status = ItemStatus::Available;
            patron.actor_type = ActorType::Patron;
            patron.status = ActorStatus::Blocked;

            let command = CheckoutCommand {
                item_id: item.id.clone(),
                patron_id: patron.id.clone(),
                timestamp,
                correlation_id,
            };

            let result = ItemAggregate::process(&item, &patron, command);

            prop_assert_eq!(result, Err(CheckoutError::PatronBlocked));
        }

        #[test]
        fn fine_never_exceeds_max(days in any::<u32>(), rate in 0i64..=100_000i64) {
            let fine = calculate_overdue_fine(days, rate);

            prop_assert!(fine <= MAX_FINE_CENTS);
        }
    }

    #[test]
    fn calculate_overdue_fine_returns_raw_value_below_cap() {
        assert_eq!(calculate_overdue_fine(3, 100), 300);
    }

    #[test]
    fn calculate_overdue_fine_caps_at_max_fine() {
        assert_eq!(calculate_overdue_fine(10_000, 10), MAX_FINE_CENTS);
    }

    #[test]
    fn process_rejects_item_mismatch_before_other_checks() {
        let item = Item {
            id: ItemId("item-1".into()),
            bib_id: "bib-1".into(),
            barcode: "bc-1".into(),
            status: ItemStatus::Available,
            location: BranchId("branch-1".into()),
        };
        let patron = Actor {
            id: ActorId("patron-1".into()),
            actor_type: ActorType::Patron,
            name: "Patron".into(),
            email: None,
            status: ActorStatus::Blocked,
        };
        let command = CheckoutCommand {
            item_id: ItemId("item-2".into()),
            patron_id: patron.id.clone(),
            timestamp: 42,
            correlation_id: "corr-1".into(),
        };

        assert_eq!(
            ItemAggregate::process(&item, &patron, command),
            Err(CheckoutError::ItemMismatch)
        );
    }

    #[test]
    fn process_rejects_patron_mismatch() {
        let item = Item {
            id: ItemId("item-1".into()),
            bib_id: "bib-1".into(),
            barcode: "bc-1".into(),
            status: ItemStatus::Available,
            location: BranchId("branch-1".into()),
        };
        let patron = Actor {
            id: ActorId("patron-1".into()),
            actor_type: ActorType::Patron,
            name: "Patron".into(),
            email: None,
            status: ActorStatus::Active,
        };
        let command = CheckoutCommand {
            item_id: item.id.clone(),
            patron_id: ActorId("patron-2".into()),
            timestamp: 42,
            correlation_id: "corr-1".into(),
        };

        assert_eq!(
            ItemAggregate::process(&item, &patron, command),
            Err(CheckoutError::PatronMismatch)
        );
    }

    #[test]
    fn process_rejects_blocked_patron() {
        let item = Item {
            id: ItemId("item-1".into()),
            bib_id: "bib-1".into(),
            barcode: "bc-1".into(),
            status: ItemStatus::Available,
            location: BranchId("branch-1".into()),
        };
        let patron = Actor {
            id: ActorId("patron-1".into()),
            actor_type: ActorType::Patron,
            name: "Patron".into(),
            email: None,
            status: ActorStatus::Suspended,
        };
        let command = CheckoutCommand {
            item_id: item.id.clone(),
            patron_id: patron.id.clone(),
            timestamp: 42,
            correlation_id: "corr-1".into(),
        };

        assert_eq!(
            ItemAggregate::process(&item, &patron, command),
            Err(CheckoutError::PatronBlocked)
        );
    }

    #[test]
    fn process_rejects_unavailable_item() {
        let item = Item {
            id: ItemId("item-1".into()),
            bib_id: "bib-1".into(),
            barcode: "bc-1".into(),
            status: ItemStatus::CheckedOut,
            location: BranchId("branch-1".into()),
        };
        let patron = Actor {
            id: ActorId("patron-1".into()),
            actor_type: ActorType::Patron,
            name: "Patron".into(),
            email: None,
            status: ActorStatus::Active,
        };
        let command = CheckoutCommand {
            item_id: item.id.clone(),
            patron_id: patron.id.clone(),
            timestamp: 42,
            correlation_id: "corr-1".into(),
        };

        assert_eq!(
            ItemAggregate::process(&item, &patron, command),
            Err(CheckoutError::ItemNotAvailable)
        );
    }

    #[test]
    fn rebuild_from_empty_events_returns_default_state() {
        let aggregate = ItemAggregate::rebuild(&[]);

        assert_eq!(aggregate.item_id, None);
        assert_eq!(aggregate.status, ItemStatus::Available);
        assert_eq!(aggregate.checked_out_to, None);
        assert_eq!(aggregate.last_timestamp, None);
        assert_eq!(aggregate.last_correlation_id, None);
    }

    #[test]
    fn rebuild_applies_latest_event_state() {
        let item_id = ItemId("item-1".into());
        let patron_id = ActorId("patron-1".into());
        let events = vec![
            CirculationEvent::ItemCheckedOut {
                item_id: item_id.clone(),
                patron_id: patron_id.clone(),
                timestamp: 1,
                correlation_id: "c-1".into(),
            },
            CirculationEvent::ItemReturned {
                item_id: item_id.clone(),
                patron_id: patron_id.clone(),
                timestamp: 2,
                correlation_id: "c-2".into(),
            },
        ];

        let aggregate = ItemAggregate::rebuild(&events);

        assert_eq!(aggregate.item_id, Some(item_id));
        assert_eq!(aggregate.status, ItemStatus::Available);
        assert_eq!(aggregate.checked_out_to, None);
        assert_eq!(aggregate.last_timestamp, Some(2));
        assert_eq!(aggregate.last_correlation_id, Some("c-2".into()));
    }
}
