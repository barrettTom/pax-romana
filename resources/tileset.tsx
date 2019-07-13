<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.2" tiledversion="1.2.1" name="tileset" tilewidth="16" tileheight="16" tilecount="50" columns="10">
 <image source="tileset.png" width="160" height="80"/>
 <tile id="14">
  <properties>
   <property name="delay" type="int" value="100"/>
   <property name="scramble_delay" type="bool" value="true"/>
   <property name="entity" value="flame"/>
   <property name="keyframe" type="int" value="0"/>
  </properties>
 </tile>
 <tile id="15">
  <properties>
   <property name="spawn" value="player"/>
   <property name="visible" type="bool" value="false"/>
  </properties>
 </tile>
 <tile id="24">
  <properties>
   <property name="delay" type="int" value="100"/>
   <property name="scramble_delay" type="bool" value="true"/>
   <property name="entity" value="flame"/>
   <property name="keyframe" type="int" value="1"/>
  </properties>
 </tile>
 <tile id="25">
  <properties>
   <property name="spawn" value="peasant"/>
   <property name="visible" type="bool" value="false"/>
  </properties>
 </tile>
 <tile id="34">
  <properties>
   <property name="delay" type="int" value="100"/>
   <property name="entity" value="player-top"/>
   <property name="keyframe" type="int" value="0"/>
  </properties>
 </tile>
 <tile id="35">
  <properties>
   <property name="delay" type="int" value="100"/>
   <property name="entity" value="player-top"/>
   <property name="keyframe" type="int" value="1"/>
  </properties>
 </tile>
 <tile id="44">
  <properties>
   <property name="delay" type="int" value="100"/>
   <property name="entity" value="player-bottom"/>
   <property name="keyframe" type="int" value="0"/>
  </properties>
 </tile>
 <tile id="45">
  <properties>
   <property name="delay" type="int" value="100"/>
   <property name="entity" value="player-bottom"/>
   <property name="keyframe" type="int" value="1"/>
  </properties>
 </tile>
</tileset>
