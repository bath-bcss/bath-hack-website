#[derive(Debug, Copy, Clone)]
pub enum BathUserStatus {
    None = 0,
    UserIsStudent = 1,
    UserNotExists = 2,
    UserIsNotStudent = 3,
}

impl TryFrom<i16> for BathUserStatus {
    type Error = i16;

    fn try_from(v: i16) -> Result<Self, Self::Error> {
        match v {
            x if x == BathUserStatus::None as i16 => Ok(BathUserStatus::None),
            x if x == BathUserStatus::UserIsStudent as i16 => Ok(BathUserStatus::UserIsStudent),
            x if x == BathUserStatus::UserIsNotStudent as i16 => {
                Ok(BathUserStatus::UserIsNotStudent)
            }
            x if x == BathUserStatus::UserNotExists as i16 => Ok(BathUserStatus::UserNotExists),
            x => Err(x),
        }
    }
}
