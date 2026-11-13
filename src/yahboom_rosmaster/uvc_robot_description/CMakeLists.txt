cmake_minimum_required(VERSION 3.20)
project(uvc_robot_description)

find_package(ament_cmake REQUIRED)

install(DIRECTORY launch meshes urdf rviz
  DESTINATION share/${PROJECT_NAME})
install(FILES README.md DESTINATION share/${PROJECT_NAME})

ament_package()
