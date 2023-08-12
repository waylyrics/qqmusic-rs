/// 获取歌词
pub mod lyric;
/// 歌曲信息
pub mod song;
/// Cookies
pub mod user;
/// 搜索
pub mod search;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SongId<'a> {
    Songid(&'a str),
    Songmid(&'a str),
}

impl<'a> From<SongId<'a>> for (&'static str, &'a str) {
    fn from(value: SongId<'a>) -> Self {
        match value {
            SongId::Songid(id) => ("songid", id),
            SongId::Songmid(mid) => ("songmid", mid),
        }
    }
}

impl std::fmt::Display for SongId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SongId::Songid(id) => {
                f.write_str("songid=")?;
                f.write_str(id)?;
            }
            SongId::Songmid(mid) => {
                f.write_str("songmid=")?;
                f.write_str(mid)?;
            }
        }
        Ok(())
    }
}
