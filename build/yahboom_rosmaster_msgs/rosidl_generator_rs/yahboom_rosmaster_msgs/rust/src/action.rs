
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_Goal {
    /// Desired angular velocity in rad/s (positive for counterclockwise, negative for clockwise)
    pub angular_velocity: f64,

    /// Desired duration of the rotation in seconds
    pub duration: f64,

}



impl Default for TimedRotation_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TimedRotation_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_Goal {
  type RmwMsg = super::action::rmw::TimedRotation_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        angular_velocity: msg.angular_velocity,
        duration: msg.duration,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      angular_velocity: msg.angular_velocity,
      duration: msg.duration,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      angular_velocity: msg.angular_velocity,
      duration: msg.duration,
    }
  }
}


// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_Result {
    /// Indicates if the rotation was completed successfully
    pub success: bool,

    /// Actual duration of the rotation in seconds
    pub actual_duration: f64,

}



impl Default for TimedRotation_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TimedRotation_Result::default())
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_Result {
  type RmwMsg = super::action::rmw::TimedRotation_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        actual_duration: msg.actual_duration,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      actual_duration: msg.actual_duration,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      actual_duration: msg.actual_duration,
    }
  }
}


// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub status: std::string::String,

}



impl Default for TimedRotation_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TimedRotation_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_Feedback {
  type RmwMsg = super::action::rmw::TimedRotation_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        elapsed_time: msg.elapsed_time,
        status: msg.status.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      elapsed_time: msg.elapsed_time,
        status: msg.status.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      elapsed_time: msg.elapsed_time,
      status: msg.status.to_string(),
    }
  }
}


// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::TimedRotation_Feedback,

}



impl Default for TimedRotation_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TimedRotation_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_FeedbackMessage {
  type RmwMsg = super::action::rmw::TimedRotation_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::TimedRotation_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::TimedRotation_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::TimedRotation_Feedback::from_rmw_message(msg.feedback),
    }
  }
}






// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::TimedRotation_Goal,

}



impl Default for TimedRotation_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TimedRotation_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_SendGoal_Request {
  type RmwMsg = super::action::rmw::TimedRotation_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::TimedRotation_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::TimedRotation_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::TimedRotation_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for TimedRotation_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TimedRotation_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_SendGoal_Response {
  type RmwMsg = super::action::rmw::TimedRotation_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for TimedRotation_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TimedRotation_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_GetResult_Request {
  type RmwMsg = super::action::rmw::TimedRotation_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimedRotation_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::TimedRotation_Result,

}



impl Default for TimedRotation_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::TimedRotation_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for TimedRotation_GetResult_Response {
  type RmwMsg = super::action::rmw::TimedRotation_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::TimedRotation_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::TimedRotation_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::TimedRotation_Result::from_rmw_message(msg.result),
    }
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






#[link(name = "yahboom_rosmaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation() -> *const std::ffi::c_void;
}

// Corresponds to yahboom_rosmaster_msgs__action__TimedRotation
#[allow(missing_docs, non_camel_case_types)]
pub struct TimedRotation;

impl rosidl_runtime_rs::Action for TimedRotation {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = TimedRotation_Goal;

  /// The result message defined in the action definition.
  type Result = TimedRotation_Result;

  /// The feedback message defined in the action definition.
  type Feedback = TimedRotation_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::TimedRotation_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::TimedRotation_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::TimedRotation_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__yahboom_rosmaster_msgs__action__TimedRotation() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::TimedRotation_Goal,
  ) -> super::action::rmw::TimedRotation_SendGoal_Request {
   super::action::rmw::TimedRotation_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::TimedRotation_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::TimedRotation_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::TimedRotation_SendGoal_Response {
   super::action::rmw::TimedRotation_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::TimedRotation_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::TimedRotation_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::TimedRotation_Feedback,
  ) -> super::action::rmw::TimedRotation_FeedbackMessage {
    let mut message = super::action::rmw::TimedRotation_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::TimedRotation_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::TimedRotation_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::TimedRotation_GetResult_Request {
   super::action::rmw::TimedRotation_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::TimedRotation_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::TimedRotation_Result,
  ) -> super::action::rmw::TimedRotation_GetResult_Response {
   super::action::rmw::TimedRotation_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::TimedRotation_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::TimedRotation_Result,
  ) {
    (response.status, response.result)
  }
}


