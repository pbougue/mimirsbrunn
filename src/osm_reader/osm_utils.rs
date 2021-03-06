// Copyright © 2016, Canal TP and/or its affiliates. All rights reserved.
//
// This file is part of Navitia,
//     the software to build cool stuff with public transport.
//
// Hope you'll enjoy and contribute to this project,
//     powered by Canal TP (www.canaltp.fr).
// Help us simplify mobility and open public transport:
//     a non ending quest to the responsive locomotion way of traveling!
//
// LICENCE: This program is free software; you can redistribute it
// and/or modify it under the terms of the GNU Affero General Public
// License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this program. If not, see
// <http://www.gnu.org/licenses/>.
//
// Stay tuned using
// twitter @navitia
// IRC #navitia on freenode
// https://groups.google.com/d/forum/navitia
// www.navitia.io

extern crate mimir;
extern crate osmpbfreader;

use geo::centroid::Centroid;
use geo::MultiPolygon;
use std::collections::BTreeMap;

pub fn get_way_coord(
    obj_map: &BTreeMap<osmpbfreader::OsmId, osmpbfreader::OsmObj>,
    way: &osmpbfreader::objects::Way,
) -> mimir::Coord {
    /*
        Returns arbitrary Coord on the way.
        A middle node is chosen as a better marker on a street
        than the first node.
    */
    let nb_nodes = way.nodes.len();
    way.nodes
        .iter()
        .skip(nb_nodes / 2)
        .filter_map(|node_id| obj_map.get(&(*node_id).into()))
        .filter_map(|obj| obj.node())
        .map(|node| mimir::Coord::new(node.lon(), node.lat()))
        .next()
        .unwrap_or_else(mimir::Coord::default)
}

pub fn make_centroid(boundary: &Option<MultiPolygon<f64>>) -> mimir::Coord {
    let coord = boundary
        .as_ref()
        .and_then(|b| b.centroid().map(|c| mimir::Coord::new(c.x(), c.y())))
        .unwrap_or_else(|| mimir::Coord::default());
    if coord.is_valid() {
        coord
    } else {
        mimir::Coord::default()
    }
}

pub fn get_osm_codes_from_tags(tags: &osmpbfreader::Tags) -> Vec<mimir::Code> {
    // read codes from osm tags
    // for the moment we only use:
    // * ISO3166 codes (mainly to get country codes)
    // * ref:* tags (to get NUTS codes, INSEE code (even if we have a custom field for them), ...)
    tags.iter()
        .filter(|(k, _)| k.starts_with("ISO3166") || k.starts_with("ref:"))
        .map(|property| mimir::Code {
            name: property.0.to_string(),
            value: property.1.to_string(),
        }).collect()
}
