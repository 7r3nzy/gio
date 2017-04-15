// This file was generated by gir (2d591cc) from gir-files (71d73f0)
// DO NOT EDIT

use Action;
use ActionGroup;
use ActionMap;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct SimpleActionGroup(Object<ffi::GSimpleActionGroup>): ActionGroup, ActionMap;

    match fn {
        get_type => || ffi::g_simple_action_group_get_type(),
    }
}

impl SimpleActionGroup {
    pub fn new() -> SimpleActionGroup {
        unsafe {
            from_glib_full(ffi::g_simple_action_group_new())
        }
    }

    //pub fn add_entries(&self, entries: /*Ignored*/&[&ActionEntry], n_entries: i32, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::g_simple_action_group_add_entries() }
    //}

    pub fn insert<T: IsA<Action>>(&self, action: &T) {
        unsafe {
            ffi::g_simple_action_group_insert(self.to_glib_none().0, action.to_glib_none().0);
        }
    }

    pub fn lookup(&self, action_name: &str) -> Option<Action> {
        unsafe {
            from_glib_none(ffi::g_simple_action_group_lookup(self.to_glib_none().0, action_name.to_glib_none().0))
        }
    }

    pub fn remove(&self, action_name: &str) {
        unsafe {
            ffi::g_simple_action_group_remove(self.to_glib_none().0, action_name.to_glib_none().0);
        }
    }
}
