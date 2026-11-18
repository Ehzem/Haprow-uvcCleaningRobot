#!/usr/bin/env python3
"""Optional Gazebo/PTY integration check: build and source both robot/Gazebo packages first.

Requires Gazebo sensor graphics support (or run under xvfb-run).
Starts an isolated test world; commands and stops only that world's robot.
"""
import json,math,os,select,signal,subprocess,sys,time
from pathlib import Path
import rclpy
from geometry_msgs.msg import Twist
from nav_msgs.msg import Odometry
from sensor_msgs.msg import JointState
from rclpy.qos import QoSProfile,ReliabilityPolicy
from ament_index_python.packages import get_package_prefix,get_package_share_directory
PACKAGE=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(PACKAGE/'scripts'))
from uvc_kinematics import DriveGeometry,command_for_key
os.environ['ROS_DOMAIN_ID']=os.environ.get('UVC_TEST_ROS_DOMAIN_ID','86');os.environ['GZ_PARTITION']='uvc_keyboard_check_'+str(os.getpid())
geometry=DriveGeometry.from_urdf(str(Path(get_package_share_directory('uvc_robot_description'))/'urdf/robots/uvc_robot.urdf'))
log=open('/tmp/uvc_keyboard_gazebo.log','w')
sim=subprocess.Popen(['ros2','launch','yahboom_rosmaster_gazebo','operating_theatre.launch.py','headless:=true','use_rviz:=false'],stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
rclpy.init();node=rclpy.create_node('verify_uvc_keyboard');state={};output=[];master=slave=None;keyboard=None
qos=QoSProfile(depth=1,reliability=ReliabilityPolicy.BEST_EFFORT)
def odom(m):state['odom']=(m.twist.twist.linear.x,m.twist.twist.angular.z)
def joints(m):
 v=dict(zip(m.name,m.velocity))
 if all(k in v for k in ('left_wheel_joint','right_wheel_joint')):state['wheels']=(v['left_wheel_joint'],v['right_wheel_joint'])
def command(m):state['command']=(m.linear.x,m.angular.z)
subs=[node.create_subscription(Odometry,'/odom',odom,qos),node.create_subscription(JointState,'/joint_states',joints,qos),node.create_subscription(Twist,'/cmd_vel',command,10)]
def spin(seconds,key=None):
 deadline=time.monotonic()+seconds;lastkey=0
 while time.monotonic()<deadline:
  if key and time.monotonic()-lastkey>.08:os.write(master,key.encode());lastkey=time.monotonic()
  rclpy.spin_once(node,timeout_sec=.02)
  if master is not None and select.select([master],[],[],0)[0]:
   try:output.append(os.read(master,65536).decode(errors='replace'))
   except OSError:pass

def pose():
 text=subprocess.check_output(['gz','model','-m','uvc_robot','--pose'],text=True,stderr=subprocess.DEVNULL,timeout=8)
 import re
 a=re.findall(r'\[([-+0-9.e ]+)\]',text)
 coordinates=[list(map(float,x.split())) for x in a if len(x.split())==3]
 return coordinates[-2],coordinates[-1]
results=[]
try:
 deadline=time.monotonic()+35
 while 'wheels' not in state or 'odom' not in state:
  spin(.1)
  if time.monotonic()>deadline:raise RuntimeError('No wheel/odom feedback: '+str(state))
 master,slave=os.openpty()
 keyboard=subprocess.Popen([sys.executable,str(Path(get_package_prefix('uvc_robot_description'))/'lib/uvc_robot_description/uvc_keyboard')],stdin=slave,stdout=slave,stderr=slave,start_new_session=True)
 spin(2)
 if keyboard.poll() is not None:raise RuntimeError('Keyboard exited: '+''.join(output))
 # A second instance must refuse to publish rather than send idle zeros.
 duplicate_master,duplicate_slave=os.openpty()
 duplicate=subprocess.Popen([sys.executable,str(Path(get_package_prefix('uvc_robot_description'))/'lib/uvc_robot_description/uvc_keyboard')],stdin=duplicate_slave,stdout=duplicate_slave,stderr=duplicate_slave,start_new_session=True)
 try:
  spin(2.5)
  duplicate.wait(timeout=3)
  duplicate_output=os.read(duplicate_master,65536).decode(errors='replace')
  assert duplicate.returncode==2,('duplicate controller was accepted',duplicate.returncode,duplicate_output)
  assert 'Another publisher is already using' in duplicate_output,duplicate_output
  assert keyboard.poll() is None,'Original controller exited'
 finally:
  if duplicate.poll() is None:
   duplicate.kill();duplicate.wait()
  os.close(duplicate_master);os.close(duplicate_slave)
 for key in ['w','s','a','d','q','e']:
  before=pose()
  spin(2,key)
  expected=command_for_key(key,.15,.4);actual=state['odom'];wheels=state['wheels'];target=geometry.inverse(*expected)
  assert abs(actual[0]-expected[0])<.02,(key,expected,actual)
  assert abs(actual[1]-expected[1])<.05,(key,expected,actual)
  assert max(abs(wheels[i]-target[i]) for i in (0,1))<.15,(key,target,wheels)
  spin(1)
  assert max(abs(v) for v in state['command'])<1e-8,('deadman',state)
  assert max(abs(v) for v in state['odom'])<.02,('stop',state)
  after=pose()
  if key=='w':assert after[0][0]-before[0][0]>.1,('forward physical pose',before,after)
  if key=='s':assert after[0][0]-before[0][0]<-.1,('reverse physical pose',before,after)
  if key=='a':assert after[1][2]-before[1][2]>.2,('left physical yaw',before,after)
  if key=='d':assert after[1][2]-before[1][2]<-.2,('right physical yaw',before,after)
  results.append({'key':key,'expected_v_yaw':expected,'odom_v_yaw':actual,'expected_joint_rad_s':target,'measured_joint_rad_s':wheels,'physical_pose_after':after})
 spin(1,'w');keyboard.send_signal(signal.SIGINT);spin(1);keyboard.wait(timeout=5)
 assert keyboard.returncode==0,keyboard.returncode
 assert max(abs(v) for v in state['command'])<1e-8,('quit did not send stop',state)
 assert max(abs(v) for v in state['odom'])<.02,('quit did not stop',state)
 print(json.dumps({'result':'PASS','cases':results,'deadman_stop':'PASS','ctrl_c_stop':'PASS','duplicate_controller_rejected':'PASS'},indent=2))
finally:
 Path('/tmp/uvc_keyboard_terminal.log').write_text(''.join(output))
 if keyboard and keyboard.poll() is None:
  keyboard.send_signal(signal.SIGTERM)
  try:keyboard.wait(timeout=5)
  except subprocess.TimeoutExpired:os.killpg(keyboard.pid,signal.SIGKILL)
 if master is not None:os.close(master)
 if slave is not None:os.close(slave)
 node.destroy_node();rclpy.shutdown()
 if sim.poll() is None:
  os.killpg(sim.pid,signal.SIGINT)
  try:sim.wait(timeout=10)
  except subprocess.TimeoutExpired:os.killpg(sim.pid,signal.SIGKILL);sim.wait()
 log.close()
