use snafu::{ensure, Backtrace, ErrorCompat, ResultExt, Snafu};

// snafu more like waifu amirite
#[derive(Debug, Snafu)]
pub enum VideoError {
    #[snafu(display("Could not get video at link {}",link))]
    GetVideoError{
        link : String,
    },
    #[snafu(display("Could not unpack value {} at link {}",value, link))]
    VideoUnpackError{
        value : String,
        link : String,
    },
    #[snafu(display("The video at link {} is a playlist and is not supported", link))]
    PlaylistError{
        link : String,
    },
    #[snafu(display("The title of video at {} contains banned term {}", link, word))]
    BannedTitleWordError{
        link : String,
        word : String,
    },
    #[snafu(display("The search term {} contains banned term {}", search, word))]
    BannedSearchWordError{
        search : String,
        word : String,
    },
    #[snafu(display("The video at link {} is currently banned", link))]
    BannedVideoError{
        link : String,
    },

}

