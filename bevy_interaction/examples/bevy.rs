use bevy::prelude::*;
use bevy::app::CoreStage::PreUpdate;
use std::io::prelude::*;

use bevy_prototype_interaction::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(CursorPosition::new())
    .add_startup_system(startup)
    .add_system_to_stage(PreUpdate, cursor_system)
    .add_system(interaction)
    .run();
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Debug)]
pub struct CursorPosition {
  pub coords: Option<Vec2>,
}

impl CursorPosition {
  pub fn new() -> Self {
    Self { coords: None }
  }
}

#[derive(Component)]
struct Red;

fn startup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  commands.spawn_bundle(OrthographicCameraBundle {
    transform: Transform::from_xyz(0.0, 0.0, 500.0),
    ..OrthographicCameraBundle::new_2d()
  }).insert(MainCamera);

  const PI: f32 = std::f32::consts::PI;

  spawn_arc(&mut commands, &asset_server, PI/4.0);
  spawn_arc(&mut commands, &asset_server, 3.0*PI/4.0);
  spawn_arc(&mut commands, &asset_server, 5.0*PI/4.0);
  spawn_arc(&mut commands, &asset_server, 7.0*PI/4.0);
}

fn spawn_arc(
  commands: &mut Commands,
  asset_server: &Res<AssetServer>,
  rotation: f32,
) {
  let region = {
    let mut r = Region::from_circle(Vec2::new(0.0, 0.0), 200.0);
    r.difference(Region::from_circle(Vec2::new(0.0, 0.0), 100.0));
    r.intersection(Region::from_rect(Vec2::new(5.0, 5.0), Vec2::new(200.0, 200.0)));
    r.rotate(rotation);
    r
  };

  commands.spawn()
    .insert(region)
    .insert(Transform::from_rotation(Quat::from_rotation_z(rotation)))
    .insert(GlobalTransform::default())
    .with_children(|parent| {
      parent.spawn()
        .insert_bundle(SpriteBundle {
          texture: asset_server.load("arc.png"),
          transform: Transform::from_xyz(100.0, 100.0, 0.0),
          ..Default::default()
        });
      parent.spawn()
        .insert(Red)
        .insert_bundle(SpriteBundle {
          texture: asset_server.load("arc_red.png"),
          transform: Transform::from_xyz(100.0, 100.0, 0.1),
          ..Default::default()
        })
        .insert(Visibility { is_visible: false });
    });
}

fn cursor_system(
  wnds: Res<Windows>,
  mut cursor: ResMut<CursorPosition>,
  query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
  let (camera, camera_transform) = query.single();
  let wnd = wnds.get(camera.window).unwrap();

  cursor.coords = match wnd.cursor_position() {
    Some(screen_pos) => {
      let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
      let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
      let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
      let world_pos: Vec2 = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();

      Some(Vec2::new(world_pos.x.round(), world_pos.y.round()))
    },
    None => None
  };
}

fn interaction(
  cursor: Res<CursorPosition>,
  region_query: Query<(&Region, &Children)>,
  mut red_query: Query<&mut Visibility, With<Red>>,
) {
  for mut visibility in red_query.iter_mut() {
    visibility.is_visible = false;
  }

  if let Some(cursor) = cursor.coords {
    for (region, children) in region_query.iter() {
      if region.contains(cursor) {
        eprint!("in region      {cursor:?} \r");
        for &child in children.iter() {
          if let Some(mut visibility) = red_query.get_mut(child).ok() {
            visibility.is_visible = true;
          }
        }
      } else {
        eprint!("not in region  {cursor:?} \r");
      }
    }
    std::io::stderr().flush().ok().expect("Could not flush stdout");
  }
}

// macro_rules! make_region {
//   (rect /* vec,vec, repeat*/ ) => {
//     /* build a closure */
//   };
// }

//
// let region = make_region!{
//   a = circle(Vec2::new(0.0, 0.0), 200.0)
//         .difference(rect())
// }
//
// a = circle
// name a region
// <name> = <region>
//
// create a region from an operation over two subregions
// <region> = <operation(<region>, <region>)>
// <region> = <primitive>
// <region> = <operation(<name>, <name>)
//
// <primitive> = 'primitive(args..)
//
// <operation(<name>)>
// combine operation over named region

// macro_rules! primitive {
//   (rect ($a:literal $a:literal) ($a:literal $a:literal)) => {
//     // TODO
//   };
// }
