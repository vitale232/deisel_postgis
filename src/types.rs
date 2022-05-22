use crate::sql_types::*;
use diesel::deserialize::{self, FromSql};
use diesel::not_none;
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use postgis::ewkb::Point;
use std::convert::From;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[sql_type = "Geography"]
pub struct GeogPoint {
    pub x: f64,
    pub y: f64,
    pub srid: Option<i32>,
}

impl From<Point> for GeogPoint {
    fn from(p: Point) -> Self {
        let Point { x, y, srid } = p;
        Self { x, y, srid }
    }
}
impl From<GeogPoint> for Point {
    fn from(p: GeogPoint) -> Self {
        let GeogPoint { x, y, srid } = p;
        Self { x, y, srid }
    }
}

impl FromSql<Geography, Pg> for GeogPoint {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        use postgis::ewkb::EwkbRead;
        use std::io::Cursor;

        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        Ok(Point::read_ewkb(&mut rdr)?.into())
    }
}

impl ToSql<Geography, Pg> for GeogPoint {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
        Point::from(*self).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)
    }
}
