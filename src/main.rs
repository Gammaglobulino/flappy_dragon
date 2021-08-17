use bracket_lib::prelude::*;

pub mod player;
use player::Player;

pub mod obstacles;
use obstacles::Obstacle;

const SCREEN_WIDTH:i32=80;
const SCREEN_HEIGHT:i32=50;
const FRAME_DURATION:f32=75.0;


fn main()->BError{
    let context=BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context,State::new())
}





enum GameMode{
    Menu,
    Playing,
    End,
}
pub struct State{
    player:Player,
    obstacle:Obstacle,
    frame_rate:f32,
    mode:GameMode,
    score:i32,
}

impl State{
    pub fn new()->Self{
        State{
            player:Player::new(5,25),
            frame_rate:0.0,
            mode:GameMode::Menu,
            obstacle:Obstacle::new(SCREEN_WIDTH,0),
            score:0,

        }
    }
}

impl GameState for State{
    fn tick(&mut self,ctx:&mut BTerm){
        match self.mode{
            GameMode::Menu=> self.main_menu(ctx),
            GameMode::Playing=>self.play(ctx),
            GameMode::End=>self.dead(ctx),
        }
    }
}

impl State {
    fn play(&mut self, ctx: &mut BTerm){
        ctx.cls_bg(NAVY);
        self.frame_rate+=ctx.frame_time_ms;
        if self.frame_rate>FRAME_DURATION{
            self.frame_rate=0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space)=ctx.key{
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0,0,"Press SPACE to flap");
        ctx.print(0,1,&format!("Score : {} ", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x{
            self.score+=1;
            self.obstacle=Obstacle::new(self.player.x+SCREEN_WIDTH, self.score);
        }


        if self.player.y> SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player){
            self.mode=GameMode::End;
        }
          
    }
    fn restart(&mut self){
        self.player=Player::new(5, 25);
        self.frame_rate=0.0;
        self.obstacle=Obstacle::new(SCREEN_WIDTH,0);
        self.score=0;
        self.mode=GameMode::Playing;
    }
    fn main_menu(&mut self,ctx:&mut BTerm ){
        ctx.cls();
        ctx.print_centered(5,"Welcome to Flappy Dragon");
        ctx.print_centered(8,"(P) Play Game");
        ctx.print_centered(9,"(Q) Quit Game");
        if let Some(key)=ctx.key{
            match key{
                VirtualKeyCode::P=> self.restart(),
                VirtualKeyCode::Q=> ctx.quitting=true,
                _=>{},    
            }
        }

    }
    fn dead(&mut self,ctx:&mut BTerm){
        ctx.cls();
        ctx.print_centered(5,"You're dead mate!");
        ctx.print_centered(6,&format!("You earned {} points",self.score));
        ctx.print_centered(8,"(P) Play Again");
        ctx.print_centered(9,"(Q) Quit Game");
        if let Some(key)=ctx.key{
            match key{
                VirtualKeyCode::P=> self.restart(),
                VirtualKeyCode::Q=> ctx.quitting=true,
                _=>{}
            }
        }
    }
}