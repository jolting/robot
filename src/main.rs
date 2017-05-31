extern crate carboxyl;
use std::f64::consts::PI;

#[derive(Copy, Clone)]
struct HoloCommand
{
    vx: f64,
    vy: f64,
     w: f64,
}

#[derive(Copy, Clone)]
struct Point
{
    x: f64,
    y: f64
}

type Obstacles = Vec<Point>;
type TpObstacles = Vec<Point>;

type Score  = f64;
type Angle  = f64;
type Kvalue = i32;

fn main() {
    let obstacles_sink   = carboxyl::Sink::new();
    let obstacles   = obstacles_sink.stream().hold(None);
    let command     = obstacles.map(reactiveNav);

    //obstacles_sink.send(Some(3.0));
    //command.sample();
}
fn reactiveNav (obstacles : Option<Obstacles>) -> Option<HoloCommand>

{
    match obstacles
    {
       Some(obs) => {
           let tp_obstacles = ws_to_tp(obs);
           Some(HoloCommand { vx: 0.0, vy: 0.0, w: 0.0})
       },
       None => None,
    }
}

const V_MAX: f64 = 10.0;
const W_MAX: f64 = 10.0;
const K: f64 = 0.0;
const TURNING_RADIUS_REFERENCE: f64 = 0.001;
const REF_DISTANCE : f64=6.0;

fn ws_to_tp(obstacles : Obstacles) -> Vec<Option<(f64,i32)>>
{
    let  Rmin = (V_MAX/W_MAX).abs();

    obstacles.into_iter().map(|point|
        {
            let Point {x, y} = point;
            let output = if y!=0.0
            {
                let R = (x*x+y*y)/(2.0*y);

                let theta = if K>0.0
                {
                    if y>0.0
                    {
                        x.atan2(R.abs()-y )
                    }
                    else
                    {
            	       x.atan2(y+R.abs() )
                   }
               }
               else
               {
                   if y>0.0
                   {
                       (-x).atan2(R.abs()-y )
                   }
                   else
                   {
                       (-x).atan2(y+R.abs() )
                   }
               };

               // Arc length must be possitive [0,2*pi]
               let wrappedTheta = wrapTo2Pi(theta);

               // Distance thru arc:
               let d = wrappedTheta * (R.abs() +TURNING_RADIUS_REFERENCE);

               if R.abs() < Rmin
               {
                   None
               }
               else
               {
                   let a = PI* V_MAX / (W_MAX*R);
                   let k = alpha2index(a);
                   Some((k, d))
               }
           }
           else if x.signum() == K.signum()
           {
                let k = alpha2index(0.0);
                let d = x;
                Some((k,d))
            }
            else
            {
                None
            };

            // Normalize:
            match output
            {
                Some((k,d)) => Some((d / REF_DISTANCE, k)),
                None => None
            }
        }
    ).collect()
}

fn alpha2index(a : Angle) -> i32
{
    0
}

fn wrapTo2Pi(a : Angle) -> Angle
{
    if a < 0.0
    {
        a + 2.0 * PI
    }
    else if a >= 2.0*PI
    {
        a - 2.0 * PI
    }
    else
    {
        a
    }
}

fn evaluatePaths(obstacles: TpObstacles) -> Vec<(Score, Kvalue)>
{
    vec![(0.0, 0)]
}
