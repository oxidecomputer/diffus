use crate::{
    edit::{self, enm},
    Diffable,
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

macro_rules! struct_impl {
    ($($typ:ty),*) => {
        $(
            impl<'a> Diffable<'a> for $typ {
                type Diff = (&'a $typ, &'a $typ);

                fn diff(&'a self, other: &'a Self) -> edit::Edit<Self> {
                    if self == other {
                        edit::Edit::Copy(self)
                    } else {
                        edit::Edit::Change((self, other))
                    }
                }
            }
        )*
    }
}

struct_impl! { Ipv4Addr, Ipv6Addr,  SocketAddrV4, SocketAddrV6}

macro_rules! ip_impl {
    ($($typ:tt),*) => {
        $(
            impl<'a> Diffable<'a> for $typ {
                type Diff = enm::Edit<'a, Self, (&'a Self, &'a Self)>;

                fn diff(&'a self, other: &'a Self) -> edit::Edit<Self> {
                    match (self, other) {
                        ($typ::V4(a), $typ::V4(b)) => match a.diff(&b) {
                            edit::Edit::Copy(_) => edit::Edit::Copy(self),
                            edit::Edit::Change(_) => {
                                edit::Edit::Change(enm::Edit::AssociatedChanged((self, other)))
                            }
                        },
                        ($typ::V6(a), $typ::V6(b)) => match a.diff(&b) {
                            edit::Edit::Copy(_) => edit::Edit::Copy(self),
                            edit::Edit::Change(_) => {
                                edit::Edit::Change(enm::Edit::AssociatedChanged((self, other)))
                            }
                        },
                        _ => edit::Edit::Change(enm::Edit::VariantChanged(self, other)),
                    }
                }
            }
        )*
    }
}

ip_impl! { IpAddr, SocketAddr }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_copy() {
        let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let localhost_v6 = IpAddr::V6(Ipv6Addr::LOCALHOST);
        assert!(localhost_v4.clone().diff(&localhost_v4).is_copy());
        assert!(localhost_v6.clone().diff(&localhost_v6).is_copy());
        assert!(Some(3).diff(&Some(3)).is_copy());
    }

    #[test]
    fn associate_change() {
        let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let not_localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2));
        let localhost_v6 = IpAddr::V6(Ipv6Addr::LOCALHOST);
        let not_localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 2));
        if let Some(enm::Edit::AssociatedChanged((&a, &b))) =
            localhost_v4.diff(&not_localhost_v4).change()
        {
            assert_eq!(a, localhost_v4);
            assert_eq!(b, not_localhost_v4);
            assert_ne!(a, b);
        } else {
            unreachable!();
        }

        if let Some(enm::Edit::AssociatedChanged((&a, &b))) =
            localhost_v6.diff(&not_localhost_v6).change()
        {
            assert_eq!(a, localhost_v6);
            assert_eq!(b, not_localhost_v6);
            assert_ne!(a, b);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn variant_changed() {
        let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let localhost_v6 = IpAddr::V6(Ipv6Addr::LOCALHOST);
        if let Some(enm::Edit::VariantChanged(&a, &b)) = localhost_v4.diff(&localhost_v6).change() {
            assert_eq!(a, localhost_v4);
            assert_eq!(b, localhost_v6);
        } else {
            unreachable!();
        }
    }
}
