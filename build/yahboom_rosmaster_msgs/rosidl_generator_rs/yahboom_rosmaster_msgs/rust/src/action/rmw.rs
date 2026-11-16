
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_Goal() -> *const std::ffi::c_void;
}

#[link(name = "yahboom_rosmaster_msgs__rosidl_generator_c")]
extern "C" {
    fn yahboom_rosmaster_msgs__action__TimedRotation_Goal__init(msg: *mut TimedRotation_Goal) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_Goal>, size: usize) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_Goal>);
    fn yahboom_rosmaster_msgs__action__TimedRotation_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TimedRotation_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_Goal>) -> bool;
}

// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_Goal {
    /// Desired angular velocity in rad/s (positive for counterclockwise, negative for clockwise)
    pub angular_velocity: f64,

    /// Desired duration of the rotation in seconds
    pub duration: f64,

}



impl Default for TimedRotation_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !yahboom_rosmaster_msgs__action__TimedRotation_Goal__init(&mut msg as *mut _) {
        panic!("Call to yahboom_rosmaster_msgs__action__TimedRotation_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TimedRotation_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TimedRotation_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "yahboom_rosmaster_msgs/action/TimedRotation_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_Goal() }
  }
}


#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_Result() -> *const std::ffi::c_void;
}

#[link(name = "yahboom_rosmaster_msgs__rosidl_generator_c")]
extern "C" {
    fn yahboom_rosmaster_msgs__action__TimedRotation_Result__init(msg: *mut TimedRotation_Result) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_Result>, size: usize) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_Result>);
    fn yahboom_rosmaster_msgs__action__TimedRotation_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TimedRotation_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_Result>) -> bool;
}

// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_Result {
    /// Indicates if the rotation was completed successfully
    pub success: bool,

    /// Actual duration of the rotation in seconds
    pub actual_duration: f64,

}



impl Default for TimedRotation_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !yahboom_rosmaster_msgs__action__TimedRotation_Result__init(&mut msg as *mut _) {
        panic!("Call to yahboom_rosmaster_msgs__action__TimedRotation_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TimedRotation_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TimedRotation_Result where Self: Sized {
  const TYPE_NAME: &'static str = "yahboom_rosmaster_msgs/action/TimedRotation_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_Result() }
  }
}


#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "yahboom_rosmaster_msgs__rosidl_generator_c")]
extern "C" {
    fn yahboom_rosmaster_msgs__action__TimedRotation_Feedback__init(msg: *mut TimedRotation_Feedback) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_Feedback>, size: usize) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_Feedback>);
    fn yahboom_rosmaster_msgs__action__TimedRotation_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TimedRotation_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_Feedback>) -> bool;
}

// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_Feedback {
    /// Elapsed time since the start of the rotation in seconds
    pub elapsed_time: f64,

    /// Current status of the rotation:
    /// GOAL_RECEIVED: The action server has received a goal
    /// ROTATING: The rotation is in progress
    /// GOAL_SUCCEEDED: The rotation has been completed successfully
    /// GOAL_ABORTED: The rotation has been aborted due to an error or exceptional condition
    /// GOAL_CANCELED: The client has canceled the rotation goal
    pub status: rosidl_runtime_rs::String,

}



impl Default for TimedRotation_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !yahboom_rosmaster_msgs__action__TimedRotation_Feedback__init(&mut msg as *mut _) {
        panic!("Call to yahboom_rosmaster_msgs__action__TimedRotation_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TimedRotation_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TimedRotation_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "yahboom_rosmaster_msgs/action/TimedRotation_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_Feedback() }
  }
}


#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "yahboom_rosmaster_msgs__rosidl_generator_c")]
extern "C" {
    fn yahboom_rosmaster_msgs__action__TimedRotation_FeedbackMessage__init(msg: *mut TimedRotation_FeedbackMessage) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_FeedbackMessage>, size: usize) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_FeedbackMessage>);
    fn yahboom_rosmaster_msgs__action__TimedRotation_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TimedRotation_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_FeedbackMessage>) -> bool;
}

// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::TimedRotation_Feedback,

}



impl Default for TimedRotation_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !yahboom_rosmaster_msgs__action__TimedRotation_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to yahboom_rosmaster_msgs__action__TimedRotation_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TimedRotation_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TimedRotation_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "yahboom_rosmaster_msgs/action/TimedRotation_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_FeedbackMessage() }
  }
}




#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "yahboom_rosmaster_msgs__rosidl_generator_c")]
extern "C" {
    fn yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Request__init(msg: *mut TimedRotation_SendGoal_Request) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_SendGoal_Request>, size: usize) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_SendGoal_Request>);
    fn yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TimedRotation_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_SendGoal_Request>) -> bool;
}

// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::TimedRotation_Goal,

}



impl Default for TimedRotation_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TimedRotation_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TimedRotation_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "yahboom_rosmaster_msgs/action/TimedRotation_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Request() }
  }
}


#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "yahboom_rosmaster_msgs__rosidl_generator_c")]
extern "C" {
    fn yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Response__init(msg: *mut TimedRotation_SendGoal_Response) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_SendGoal_Response>, size: usize) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_SendGoal_Response>);
    fn yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TimedRotation_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_SendGoal_Response>) -> bool;
}

// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for TimedRotation_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TimedRotation_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TimedRotation_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "yahboom_rosmaster_msgs/action/TimedRotation_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Response() }
  }
}


#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "yahboom_rosmaster_msgs__rosidl_generator_c")]
extern "C" {
    fn yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Request__init(msg: *mut TimedRotation_GetResult_Request) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_GetResult_Request>, size: usize) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_GetResult_Request>);
    fn yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TimedRotation_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_GetResult_Request>) -> bool;
}

// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for TimedRotation_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TimedRotation_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TimedRotation_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "yahboom_rosmaster_msgs/action/TimedRotation_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Request() }
  }
}


#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "yahboom_rosmaster_msgs__rosidl_generator_c")]
extern "C" {
    fn yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Response__init(msg: *mut TimedRotation_GetResult_Response) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_GetResult_Response>, size: usize) -> bool;
    fn yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_GetResult_Response>);
    fn yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TimedRotation_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<TimedRotation_GetResult_Response>) -> bool;
}

// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::TimedRotation_Result,

}



impl Default for TimedRotation_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TimedRotation_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TimedRotation_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "yahboom_rosmaster_msgs/action/TimedRotation_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Response() }
  }
}






#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct TimedRotation_SendGoal;

impl rosidl_runtime_rs::Service for TimedRotation_SendGoal {
    type Request = TimedRotation_SendGoal_Request;
    type Response = TimedRotation_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_SendGoal() }
    }
}




#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct TimedRotation_GetResult;

impl rosidl_runtime_rs::Service for TimedRotation_GetResult {
    type Request = TimedRotation_GetResult_Request;
    type Response = TimedRotation_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation_GetResult() }
    }
}


