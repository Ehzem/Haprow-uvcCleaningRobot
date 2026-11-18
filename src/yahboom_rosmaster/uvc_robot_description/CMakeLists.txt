cmake_minimum_required(VERSION 3.20)
project(uvc_robot_description)

find_package(ament_cmake REQUIRED)

install(DIRECTORY launch meshes urdf rviz
  DESTINATION share/${PROJECT_NAME})
install(FILES README.md DESTINATION share/${PROJECT_NAME})
install(PROGRAMS scripts/uvc_keyboard.py
  DESTINATION lib/${PROJECT_NAME} RENAME uvc_keyboard)
install(FILES scripts/uvc_kinematics.py DESTINATION lib/${PROJECT_NAME})

ament_package()
