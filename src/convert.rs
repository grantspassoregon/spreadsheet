//! The `convert` module contains the `Convert` type, which provides methods for converting from
//! one spatial type to another.  There are probably more official and efficient ways to convert
//! between these libraries using their own APIs, but until I work those out, this gets the job
//! done.  Wrap the spatial object of interest in a `Convert` using the constructor [`Convert::new`], then use of the conversion impls to output the desired type.
use galileo::galileo_types::cartesian::{CartesianPoint2d, Point2d};
use galileo::galileo_types::impls::ClosedContour;
use geo::algorithm::bounding_rect::BoundingRect;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shapefile::record::traits::HasXY;
use std::fmt::Debug;

/// The `Convert` struct holds conversion methods for spatial data.  Acts as a wrapper around a
/// type T, offering conversion methods from T -> U.
#[derive(Debug, Clone, derive_more::Deref, derive_more::DerefMut)]
pub struct Convert<T: Debug + Clone>(pub T);

impl<T: Debug + Clone> Convert<T> {
    /// Creates a new `Convert` struct by wrapping the data `from` and returning Self.
    pub fn new(from: T) -> Self {
        Convert(from)
    }

    /// Accessor method for the data T wrapped in a `Convert`.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl Convert<geo_types::MultiPolygon> {
    /// The `multipolygon` method converts from [`geo_types::MultiPolygon`] to
    /// [`galileo::galileo_types::impls::MultiPolygon<Point2d>`].
    pub fn multipolygon(self) -> galileo::galileo_types::impls::MultiPolygon<Point2d> {
        let conv = self
            .0
            .iter()
            .map(|v| Convert::new(v.clone()))
            .collect::<Vec<Convert<geo_types::Polygon>>>();
        let parts = conv
            .par_iter()
            .cloned()
            .map(|v| v.polygon())
            .collect::<Vec<galileo::galileo_types::impls::Polygon<Point2d>>>();
        galileo::galileo_types::impls::MultiPolygon { parts }
    }

    /// The `bounded_multipolygon` method converts from a [`geo_types::MultiPolygon`] to a
    /// [`galileo::galileo_types::MultiPolygon<Point2d>`],  including the bounding rectangle
    /// [`galileo::galileo_types::cartesian::Rect<f64>`].
    pub fn bounded_multipolygon(
        self,
    ) -> (
        galileo::galileo_types::impls::MultiPolygon<Point2d>,
        galileo::galileo_types::cartesian::Rect<f64>,
    ) {
        let mut boundaries = Vec::new();
        let conv = self
            .0
            .iter()
            .map(|v| Convert::new(v.clone()))
            .collect::<Vec<Convert<geo_types::Polygon>>>();
        let parts = conv
            .iter()
            .cloned()
            .map(|v| {
                let (poly, bounds) = v.bounded_polygon();
                boundaries.push(bounds);
                poly
            })
            .collect::<Vec<galileo::galileo_types::impls::Polygon<Point2d>>>();

        let mut xmin = f64::MAX;
        let mut ymin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymax = f64::MIN;

        for rect in boundaries.into_iter().flatten() {
            let x_min = rect.x_min();
            if x_min < xmin {
                xmin = x_min;
            }
            let y_min = rect.y_min();
            if y_min < ymin {
                ymin = y_min;
            }

            let x_max = rect.x_max();
            if x_max > xmax {
                xmax = x_max;
            }
            let y_max = rect.y_max();
            if y_max > ymax {
                ymax = y_max;
            }
        }

        let bounds = galileo::galileo_types::cartesian::Rect::new(xmin, ymin, xmax, ymax);

        (
            galileo::galileo_types::impls::MultiPolygon { parts },
            bounds,
        )
    }
}

impl Convert<geo_types::Geometry> {
    /// The `geojson_value` method converts a [`geo_types::Geometry`] into a [`geojson::Value`].
    pub fn geojson_value(self) -> geojson::Value {
        match self.clone().into_inner() {
            geo_types::Geometry::Point(x) => geojson::Value::from(&x),
            geo_types::Geometry::Line(x) => geojson::Value::from(&x),
            geo_types::Geometry::LineString(x) => geojson::Value::from(&x),
            geo_types::Geometry::Polygon(x) => geojson::Value::from(&x),
            geo_types::Geometry::MultiPoint(x) => geojson::Value::from(&x),
            geo_types::Geometry::MultiLineString(x) => geojson::Value::from(&x),
            geo_types::Geometry::MultiPolygon(x) => geojson::Value::from(&x),
            geo_types::Geometry::GeometryCollection(x) => geojson::Value::from(&x),
            geo_types::Geometry::Rect(x) => geojson::Value::from(&x),
            geo_types::Geometry::Triangle(x) => geojson::Value::from(&x),
        }
    }

    /// The `geojson_geometry` method converts a [`geo_types::Geometry`] to a
    /// [`geojson::Geometry`].
    pub fn geojson_geometry(self) -> geojson::Geometry {
        let conv = Convert::new(self.geojson_value());
        geojson::Geometry::new(conv.into_inner())
    }

    /// The `geojson_feature` method converts a [`geo_types::Geometry`] to a [`geojson::Feature`].
    pub fn geojson_feature(self) -> geojson::Feature {
        let conv = Convert::new(self.geojson_geometry()).into_inner();
        geojson::Feature::from(conv)
    }
}

impl Convert<geo::geometry::MultiPolygon> {
    /// The `geo_to_multipolygon` method converts from a [`geo::geometry::MultiPolygon`] to a
    /// [`galileo::galileo_types::impls::MultiPolygon<Point2d>`].
    pub fn geo_to_multipolygon(self) -> galileo::galileo_types::impls::MultiPolygon<Point2d> {
        let parts = self
            .0
            .iter()
            .map(|v| Convert::new(v.clone()).polygon())
            .collect::<Vec<galileo::galileo_types::impls::Polygon<Point2d>>>();
        galileo::galileo_types::impls::MultiPolygon { parts }
    }
}

impl Convert<geo_types::Polygon> {
    /// The `polgyon` method converts from a [`geo_types::Polygon`] to a
    /// [`galileo::galileo_types::impls::Polygon<Point2d>`].
    pub fn polygon(self) -> galileo::galileo_types::impls::Polygon<Point2d> {
        let (e, i) = self.0.into_inner();
        let ext = Convert::new(e).contour();
        let mut poly: galileo::galileo_types::impls::Polygon<Point2d> = ext.into();
        let mut int = Vec::new();
        if !i.is_empty() {
            for item in i {
                int.push(Convert::new(item).contour());
            }
        }
        poly.inner_contours = int;
        poly
    }

    /// The `bounded_polgyon` method converts from a [`geo_types::Polygon`] to a
    /// [`galileo::galileo_types::impls::Polygon<Point2d>`], including the bounding rectangle
    /// [`galileo::galileo_types::cartesian::Rect<f64>`].
    pub fn bounded_polygon(
        self,
    ) -> (
        galileo::galileo_types::impls::Polygon<Point2d>,
        Option<galileo::galileo_types::cartesian::Rect<f64>>,
    ) {
        let ext = self.0.exterior();
        let conv = Convert::new(ext.clone()).bounds();
        if let Some(rect) = conv {
            let min = rect.min();
            let xmin = min.x();
            let ymin = min.y();
            let max = rect.max();
            let xmax = max.x();
            let ymax = max.y();
            let bounds = galileo::galileo_types::cartesian::Rect::new(xmin, ymin, xmax, ymax);
            (self.polygon(), Some(bounds))
        } else {
            (self.polygon(), None)
        }
    }
}

impl Convert<shapefile::record::polygon::Polygon> {
    /// The `geo_polygons` method converts from a [`shapefile::record::polygon::Polygon`] to a
    /// vector of type [`geo::geometry::Polygon`].
    pub fn geo_polygons(self) -> Vec<geo::geometry::Polygon> {
        let mut polys = Vec::new();
        let mut outer = None;
        let mut inner = Vec::new();
        for ring in self.0.into_inner() {
            match ring.clone() {
                shapefile::record::polygon::PolygonRing::Outer(_) => match outer {
                    Some(x) => {
                        let poly = geo::geometry::Polygon::new(x, inner);
                        polys.push(poly);
                        outer = None;
                        inner = Vec::new();
                    }
                    None => {
                        let conv = Convert::new(ring);
                        let line = conv.geo_linestring();
                        outer = Some(line);
                    }
                },
                shapefile::record::polygon::PolygonRing::Inner(_) => {
                    let conv = Convert::new(ring);
                    let line = conv.geo_linestring();
                    inner.push(line);
                }
            }
        }
        if polys.is_empty() {
            if let Some(ring) = outer {
                polys.push(geo::geometry::Polygon::new(ring, inner));
            }
        }

        polys
    }
}

impl Convert<shapefile::record::polygon::GenericPolygon<shapefile::record::point::PointZ>> {
    /// The `geo_polygons` method converts from a [`shapefile::record::polygon::GenericPolygon`] to a
    /// vector of type [`geo::geometry::Polygon`].
    pub fn geo_polygons(self) -> Vec<geo::geometry::Polygon> {
        tracing::info!("Calling convert to multipolygon.");
        let mut polys = Vec::new();
        let mut outer = None;
        let mut inner = Vec::new();
        for ring in self.0.into_inner() {
            match ring.clone() {
                shapefile::record::polygon::PolygonRing::Outer(_) => match outer {
                    Some(x) => {
                        let poly = geo::geometry::Polygon::new(x, inner);
                        polys.push(poly);
                        outer = None;
                        inner = Vec::new();
                    }
                    None => {
                        let conv = Convert::new(ring);
                        let line = conv.geo_linestring();
                        outer = Some(line);
                    }
                },
                shapefile::record::polygon::PolygonRing::Inner(_) => {
                    let conv = Convert::new(ring);
                    let line = conv.geo_linestring();
                    inner.push(line);
                }
            }
        }
        if polys.is_empty() {
            if let Some(ring) = outer {
                polys.push(geo::geometry::Polygon::new(ring, inner));
            }
        }

        polys
    }
}

impl Convert<shapefile::record::polygon::PolygonRing<shapefile::record::point::Point>> {
    /// The `geo_linestring` method converts from a
    /// [`shapefile::record::polygon::PolygonRing<shapefile::record::point::Point>`] to a
    /// [`geo::geometry::LineString`].
    pub fn geo_linestring(self) -> geo::geometry::LineString {
        let mut pts = Vec::new();
        for i in self.0.into_inner() {
            let convert = Convert::new(i);
            let pt: geo_types::Coord = convert.geo_coord();
            pts.push(pt);
        }
        geo::geometry::LineString::new(pts)
    }
}

impl Convert<shapefile::record::polygon::PolygonRing<shapefile::record::point::PointZ>> {
    /// The `geo_linestring` method converts from a
    /// [`shapefile::record::polygon::PolygonRing<shapefile::record::point::PointZ>`] to a
    /// [`geo::geometry::LineString`].
    pub fn geo_linestring(self) -> geo::geometry::LineString {
        let mut pts = Vec::new();
        for i in self.0.into_inner() {
            let convert = Convert::new(i);
            let pt: geo_types::Coord = convert.geo_coord();
            pts.push(pt);
        }
        geo::geometry::LineString::new(pts)
    }
}

impl Convert<geo_types::LineString> {
    /// The `bounds` method returns the bounding rectangle of the [`geo_types::LineString`].
    pub fn bounds(&self) -> Option<geo::geometry::Rect<f64>> {
        self.0.bounding_rect()
    }

    /// The `contour` method converts a [`geo_types::LineString`] to a [`ClosedContour<Point2d>`].
    pub fn contour(self) -> ClosedContour<Point2d> {
        let line = self.0.into_inner();
        let points = line
            .iter()
            .map(|v| Convert::new(*v).point())
            .collect::<Vec<Point2d>>();
        ClosedContour::new(points)
    }
}

impl Convert<shapefile::record::polyline::GenericPolyline<shapefile::record::point::PointZ>> {
    /// The `multilinestring` method converts from a
    /// [`shapefile::record::polyline::GenericPolyline<shapefile::record::point::PointZ>`] to a
    /// [`geo_types::MultiLineString`].
    pub fn geotypes_multilinestring(self) -> geo_types::MultiLineString<f64> {
        let parts = self.into_inner().into_inner();
        let mut lines = Vec::new();
        for part in parts {
            let conv = Convert::new(part).geotypes_coords();
            lines.push(geo_types::LineString::new(conv));
        }
        geo_types::MultiLineString::new(lines)
    }

    /// The `geo_multilinestring` method converts from a
    /// [`shapefile::record::polyline::GenericPolyline<shapefile::record::point::PointZ>`] to a
    /// [`geo::geometry::MultiLineString`].
    pub fn geo_multilinestring(self) -> geo::geometry::MultiLineString {
        let parts = self.into_inner().into_inner();
        let mut lines = Vec::new();
        for part in parts {
            let conv = Convert::new(part).geotypes_coords();
            lines.push(geo_types::LineString::new(conv));
        }
        // the `geo` crate constructors accept many of the geo_types types
        geo::geometry::MultiLineString::new(lines)
    }

    /// The `into_geometry` method converts from a
    /// [`shapefile::record::polyline::GenericPolyline<shapefile::record::point::PointZ>`] to a
    /// [`geo::geometry::MultiLineString`] and wraps the type in a [`geo::geometry::Geometry`].
    /// We could just call `into()` on the value...
    pub fn into_geometry(self) -> geo::geometry::Geometry {
        self.geo_multilinestring().into()
    }
}

impl Convert<geo_types::Rect> {
    /// The `rect` method converts from the bounding rectangle [`geo_types::Rect`] to the bounding
    /// rectangle [`galileo::galileo_types::cartesian::Rect`].
    pub fn rect(self) -> galileo::galileo_types::cartesian::Rect {
        let min = self.0.min();
        let max = self.0.max();
        galileo::galileo_types::cartesian::Rect::new(min.x, min.y, max.x, max.y)
    }
}

impl CartesianPoint2d for Convert<geo_types::Point> {
    type Num = f64;
    fn x(&self) -> Self::Num {
        geo_types::Point::x(self.0)
    }

    fn y(&self) -> Self::Num {
        geo_types::Point::y(self.0)
    }
}

impl Convert<geo_types::Point> {
    /// The `point` method converts from a [`geo_types::Point`] to a [`Point2d`].
    pub fn point(self) -> Point2d {
        Point2d::new(self.x(), self.y())
    }

    /// The `geo_point` method converts from a [`geo_types::Point`] to a [`geo::geometry::Point`].
    pub fn geo_point(self) -> geo::geometry::Point {
        geo::point!(x: self.x(), y: self.y())
    }

    /// The `geo_coord` method converts from a [`geo_types::Point`] to a [`geo::geometry::Coord`].
    pub fn geo_coord(self) -> geo::geometry::Coord {
        geo::coord!(x: self.x(), y: self.y())
    }
}

impl CartesianPoint2d for Convert<shapefile::record::point::Point> {
    type Num = f64;
    fn x(&self) -> Self::Num {
        self.clone().into_inner().x
    }

    fn y(&self) -> Self::Num {
        self.clone().into_inner().y
    }
}

impl Convert<shapefile::record::point::Point> {
    /// The `point` method converts from a [`shapefile::record::point::Point`] to a [`Point2d`].
    pub fn point(self) -> Point2d {
        Point2d::new(self.x(), self.y())
    }

    /// The `geo_point` method converts from a [`shapefile::record::point::Point`] to a
    /// [`geo::geometry::Point`].
    pub fn geo_point(self) -> geo::geometry::Point {
        geo::point!(x: self.x(), y: self.y())
    }

    /// The `geo_coord` method converts from a [`shapefile::record::point::Point`] to a
    /// [`geo::geometry::Coord`].
    pub fn geo_coord(self) -> geo::geometry::Coord {
        geo::coord!(x: self.x(), y: self.y())
    }
}

impl Convert<Vec<shapefile::record::point::PointZ>> {
    /// The `galileo_points` method converts a vector of type [`shapefile::record::point::PointZ`]
    /// to a vector of type [`Point2d`].
    pub fn galileo_points(self) -> Vec<Point2d> {
        self.iter()
            .map(|v| Convert::new(*v).point())
            .collect::<Vec<Point2d>>()
    }

    /// The `galileo_points` method converts a vector of type [`shapefile::record::point::PointZ`]
    /// to a vector of type [`geo_types::Coord`].
    pub fn geotypes_coords(self) -> Vec<geo_types::Coord> {
        self.iter()
            .map(|v| Convert::new(*v).geotypes_coord())
            .collect::<Vec<geo_types::Coord>>()
    }
}

impl Convert<shapefile::record::point::PointZ> {
    /// The `point` method converts from a [`shapefile::record::point::PointZ`] to a [`Point2d`].
    pub fn point(self) -> Point2d {
        Point2d::new(self.0.x(), self.0.y())
    }

    /// The `geo_point` method converts from a [`shapefile::record::point::PointZ`] to a
    /// [`geo::geometry::Point`].
    pub fn geo_point(self) -> geo::geometry::Point {
        geo::point!(x: self.0.x(), y: self.0.y())
    }

    /// The `geo_coord` method converts from a [`shapefile::record::point::PointZ`] to a
    /// [`geo::geometry::Coord`].
    pub fn geo_coord(self) -> geo::geometry::Coord {
        geo::coord!(x: self.0.x(), y: self.0.y())
    }

    /// The `geotypes_coord` method converts from a [`shapefile::record::point::PointZ`] to a
    /// [`geo_types::Coord`].
    pub fn geotypes_coord(self) -> geo_types::Coord {
        geo_types::Coord {
            x: self.0.x(),
            y: self.0.y(),
        }
    }
}

impl CartesianPoint2d for Convert<geo_types::Coord> {
    type Num = f64;
    fn x(&self) -> Self::Num {
        self.0.x
    }

    fn y(&self) -> Self::Num {
        self.0.y
    }
}

impl Convert<geo_types::Coord> {
    /// The `point` method converts from a [`geo_types::Coord`] to a [`Point2d`].
    pub fn point(self) -> Point2d {
        Point2d::new(self.x(), self.y())
    }
}
