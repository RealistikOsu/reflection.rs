/// # Ranked Status
/// The ranked status enum for a specific beatmap.
pub enum Status {
    NOT_SUBMITTED,
    PENDING,
    UPDATE_AVAILABLE,
    RANKED,
    APPROVED,
    QUALIFIED,
    LOVED
}

impl Status {
    /// # To Bancho Status
    /// Converts the status to a `i8` integer representing the enum used by
    /// the osu client internally.
    pub fn to_bancho(&self) -> i8 {
        match self {
            Status::NOT_SUBMITTED => -1,
            Status::PENDING => 0,
            Status::UPDATE_AVAILABLE => 1,
            Status::RANKED => 2,
            Status::APPROVED => 3,
            Status::QUALIFIED => 4,
            Status::LOVED => 5
        }
    }

    /// # From Bancho Status
    /// Converts a client enum `i8` to a reflection.rs status enum.
    pub fn from_bancho(bancho_status: i8) -> Self {
        match bancho_status {
            -1 => Status::NOT_SUBMITTED,
            0 => Status::PENDING,
            1 => Status::UPDATE_AVAILABLE,
            2 => Status::RANKED,
            3 => Status::APPROVED,
            4 => Status::QUALIFIED,
            5 => Status::LOVED,
            // SOMETHING REALLY CURSED HAPPENED.
            _ => Status::PENDING
        }
    }

    /// # From Direct Status
    /// Converts an osu!direct enum int to a reflection.rs enum.
    pub fn from_direct(direct_status: i8) -> Self {
        match direct_status {
            0 => Status::RANKED,
            1 => Status::RANKED, // Not sure on this one.
            2 => Status::PENDING,
            3 => Status::QUALIFIED,
            5 => Status::PENDING,
            7 => Status::RANKED,
            8 => Status::LOVED,
            _ => Status::PENDING
        }
    }

    /// # To Direct Status
    /// Converts a reflection.rs enum to an `i8` integer enum used by osu!direct.
    pub fn to_direct(&self) -> i8 {
        match self {
            // Some of these may not be the prefered ones.
            Status::RANKED => 0,
            Status::PENDING => 2,
            Status::QUALIFIED => 3,
            Status::LOVED => 8,
            // Not all statuses havec a direct equivalent. Mark them as pending.
            _ => 2
        }
    }
}
