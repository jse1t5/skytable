/*
 * Created on Thu Sep 24 2020
 *
 * This file is a part of Skytable
 * Skytable (formerly known as TerrabaseDB or Skybase) is a free and open-source
 * NoSQL database written by Sayan Nandan ("the Author") with the
 * vision to provide flexibility in data modelling without compromising
 * on performance, queryability or scalability.
 *
 * Copyright (c) 2020, Sayan Nandan <ohsayan@outlook.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 *
*/

use crate::dbnet::connection::prelude::*;

action!(
    /// Returns the number of keys in the database
    fn dbsize(handle: &Corestore, con: &'a mut T, mut act: ActionIter<'a>) {
        ensure_length::<P>(act.len(), |len| len < 2)?;
        if act.is_empty() {
            let len = get_tbl_ref!(handle, con).count();
            con.write_usize(len).await?;
        } else {
            let raw_entity = unsafe { act.next().unsafe_unwrap() };
            let entity = handle_entity!(con, raw_entity);
            con.write_usize(get_tbl!(entity, handle, con).count())
                .await?;
        }
        Ok(())
    }
);
