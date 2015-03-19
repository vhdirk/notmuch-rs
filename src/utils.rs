pub trait NotmuchEnum {
    type NotmuchT;

    fn from_notmuch_t(notmuch_t: Self::NotmuchT) -> Self;
    fn to_notmuch_t(self) -> Self::NotmuchT;
}
