#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__srv__SetCleaningState_Request() -> *const std::ffi::c_void;
}

#[link(name = "yahboom_rosmaster_msgs__rosidl_generator_c")]
extern "C" {
    fn yahboom_rosmaster_msgs__srv__SetCleaningState_Request__init(msg: *mut SetCleaningState_Request) -> bool;
    fn yahboom_rosmaster_msgs__srv__SetCleaningState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCleaningState_Request>, size: usize) -> bool;
    fn yahboom_rosmaster_msgs__srv__SetCleaningState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCleaningState_Request>);
    fn yahboom_rosmaster_msgs__srv__SetCleaningState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCleaningState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCleaningState_Request>) -> bool;
}

// Corresponds to yahboom_rosmaster_msgs__srv__SetCleaningState_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCleaningState_Request {
    /// Request: true to start cleaning, false to stop
    pub desired_cleaning_state: bool,

}



impl Default for SetCleaningState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !yahboom_rosmaster_msgs__srv__SetCleaningState_Request__init(&mut msg as *mut _) {
        panic!("Call to yahboom_rosmaster_msgs__srv__SetCleaningState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCleaningState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__srv__SetCleaningState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__srv__SetCleaningState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__srv__SetCleaningState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCleaningState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCleaningState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "yahboom_rosmaster_msgs/srv/SetCleaningState_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__srv__SetCleaningState_Request() }
  }
}


#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__srv__SetCleaningState_Response() -> *const std::ffi::c_void;
}

#[link(name = "yahboom_rosmaster_msgs__rosidl_generator_c")]
extern "C" {
    fn yahboom_rosmaster_msgs__srv__SetCleaningState_Response__init(msg: *mut SetCleaningState_Response) -> bool;
    fn yahboom_rosmaster_msgs__srv__SetCleaningState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCleaningState_Response>, size: usize) -> bool;
    fn yahboom_rosmaster_msgs__srv__SetCleaningState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCleaningState_Response>);
    fn yahboom_rosmaster_msgs__srv__SetCleaningState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCleaningState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCleaningState_Response>) -> bool;
}

// Corresponds to yahboom_rosmaster_msgs__srv__SetCleaningState_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCleaningState_Response {
    /// Response: whether the request was successful
    pub success: bool,

    /// Response: information about the result
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetCleaningState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !yahboom_rosmaster_msgs__srv__SetCleaningState_Response__init(&mut msg as *mut _) {
        panic!("Call to yahboom_rosmaster_msgs__srv__SetCleaningState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCleaningState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__srv__SetCleaningState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__srv__SetCleaningState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__srv__SetCleaningState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCleaningState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCleaningState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "yahboom_rosmaster_msgs/srv/SetCleaningState_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__srv__SetCleaningState_Response() }
  }
}






#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__yahboom_rosmaster_msgs__srv__SetCleaningState() -> *const std::ffi::c_void;
}

// Corresponds to yahboom_rosmaster_msgs__srv__SetCleaningState
#[allow(missing_docs, non_camel_case_types)]
pub struct SetCleaningState;

impl rosidl_runtime_rs::Service for SetCleaningState {
    type Request = SetCleaningState_Request;
    type Response = SetCleaningState_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__yahboom_rosmaster_msgs__srv__SetCleaningState() }
    }
}


