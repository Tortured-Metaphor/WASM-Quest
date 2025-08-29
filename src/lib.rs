use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct PixelSprite {
    pixels: Vec<Vec<&'static str>>,
}

impl PixelSprite {
    fn draw(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64, scale: f64, flip_h: bool) {
        let pixel_size = scale;
        for (row_idx, row) in self.pixels.iter().enumerate() {
            for (col_idx, color) in row.iter().enumerate() {
                if *color != "" {
                    ctx.set_fill_style(&JsValue::from_str(color));
                    let px = if flip_h {
                        x + (self.pixels[0].len() as f64 - col_idx as f64 - 1.0) * pixel_size
                    } else {
                        x + col_idx as f64 * pixel_size
                    };
                    let py = y + row_idx as f64 * pixel_size;
                    ctx.fill_rect(px, py, pixel_size, pixel_size);
                }
            }
        }
    }
}

fn create_knight_sprite(frame: usize) -> PixelSprite {
    let idle = vec![
        vec!["", "", "", "#C0C0C0", "#C0C0C0", "", "", ""],
        vec!["", "", "#C0C0C0", "#C0C0C0", "#C0C0C0", "#C0C0C0", "", ""],
        vec!["", "", "#FDBCB4", "#FDBCB4", "#FDBCB4", "#FDBCB4", "", ""],
        vec!["", "", "#000000", "#FDBCB4", "#FDBCB4", "#000000", "", ""],
        vec!["", "", "#FDBCB4", "#FDBCB4", "#FDBCB4", "#FDBCB4", "", ""],
        vec!["", "#C0C0C0", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#C0C0C0", ""],
        vec!["#C0C0C0", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#C0C0C0"],
        vec!["", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#4169E1", ""],
        vec!["", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#4169E1", ""],
        vec!["", "#2E3440", "#2E3440", "", "", "#2E3440", "#2E3440", ""],
        vec!["#2E3440", "#2E3440", "#2E3440", "", "", "#2E3440", "#2E3440", "#2E3440"],
    ];
    
    let walk1 = vec![
        vec!["", "", "", "#C0C0C0", "#C0C0C0", "", "", ""],
        vec!["", "", "#C0C0C0", "#C0C0C0", "#C0C0C0", "#C0C0C0", "", ""],
        vec!["", "", "#FDBCB4", "#FDBCB4", "#FDBCB4", "#FDBCB4", "", ""],
        vec!["", "", "#000000", "#FDBCB4", "#FDBCB4", "#000000", "", ""],
        vec!["", "", "#FDBCB4", "#FDBCB4", "#FDBCB4", "#FDBCB4", "", ""],
        vec!["", "#C0C0C0", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#C0C0C0", ""],
        vec!["#C0C0C0", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#4169E1", ""],
        vec!["", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#4169E1", ""],
        vec!["", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "#4169E1", "", ""],
        vec!["", "#2E3440", "#2E3440", "", "#2E3440", "#2E3440", "", ""],
        vec!["#2E3440", "#2E3440", "", "", "", "#2E3440", "#2E3440", ""],
    ];
    
    if frame % 2 == 0 {
        PixelSprite { pixels: idle }
    } else {
        PixelSprite { pixels: walk1 }
    }
}

fn create_goblin_sprite() -> PixelSprite {
    PixelSprite {
        pixels: vec![
            vec!["", "", "#228B22", "#228B22", "#228B22", "", ""],
            vec!["", "#228B22", "#228B22", "#228B22", "#228B22", "#228B22", ""],
            vec!["", "#228B22", "#FF0000", "#228B22", "#FF0000", "#228B22", ""],
            vec!["", "#228B22", "#228B22", "#228B22", "#228B22", "#228B22", ""],
            vec!["", "#228B22", "#8B4513", "#8B4513", "#8B4513", "#228B22", ""],
            vec!["#228B22", "#8B4513", "#8B4513", "#8B4513", "#8B4513", "#8B4513", "#228B22"],
            vec!["", "#8B4513", "#8B4513", "#8B4513", "#8B4513", "#8B4513", ""],
            vec!["", "#228B22", "#228B22", "", "#228B22", "#228B22", ""],
            vec!["#228B22", "#228B22", "", "", "", "#228B22", "#228B22"],
        ]
    }
}

fn create_sword_sprite_vertical() -> PixelSprite {
    PixelSprite {
        pixels: vec![
            vec!["", "#FFFFFF", "#FFFFFF", ""],
            vec!["#D3D3D3", "#E8E8E8", "#E8E8E8", "#D3D3D3"],
            vec!["#C0C0C0", "#D3D3D3", "#D3D3D3", "#C0C0C0"],
            vec!["", "#C0C0C0", "#C0C0C0", ""],
            vec!["", "#C0C0C0", "#C0C0C0", ""],
            vec!["", "#C0C0C0", "#C0C0C0", ""],
            vec!["#8B4513", "#FFD700", "#FFD700", "#8B4513"],
            vec!["#654321", "#8B4513", "#8B4513", "#654321"],
            vec!["", "#8B4513", "#8B4513", ""],
        ]
    }
}

fn create_sword_sprite_horizontal() -> PixelSprite {
    PixelSprite {
        pixels: vec![
            vec!["", "", "", "", "", "", "", "#FFFFFF"],
            vec!["#8B4513", "#8B4513", "#C0C0C0", "#D3D3D3", "#D3D3D3", "#D3D3D3", "#E8E8E8", "#FFFFFF"],
            vec!["#654321", "#FFD700", "#C0C0C0", "#C0C0C0", "#C0C0C0", "#D3D3D3", "#D3D3D3", ""],
            vec!["#8B4513", "#8B4513", "#C0C0C0", "#D3D3D3", "#D3D3D3", "#D3D3D3", "#E8E8E8", "#FFFFFF"],
            vec!["", "", "", "", "", "", "", "#FFFFFF"],
        ]
    }
}

fn draw_pixel_platform(ctx: &CanvasRenderingContext2d, x: f64, y: f64, width: f64, height: f64) {
    let tile_size = 8.0;
    let tiles_x = (width / tile_size).ceil() as i32;
    let tiles_y = (height / tile_size).ceil() as i32;
    
    for ty in 0..tiles_y {
        for tx in 0..tiles_x {
            let px = x + tx as f64 * tile_size;
            let py = y + ty as f64 * tile_size;
            
            if ty == 0 {
                ctx.set_fill_style(&JsValue::from_str("#228B22"));
                ctx.fill_rect(px, py, tile_size, 2.0);
                ctx.set_fill_style(&JsValue::from_str("#8B4513"));
                ctx.fill_rect(px, py + 2.0, tile_size, tile_size - 2.0);
            } else {
                ctx.set_fill_style(&JsValue::from_str("#654321"));
                ctx.fill_rect(px, py, tile_size, tile_size);
            }
            
            ctx.set_stroke_style(&JsValue::from_str("#4A2C17"));
            ctx.set_line_width(0.5);
            ctx.stroke_rect(px, py, tile_size, tile_size);
        }
    }
}

fn draw_pixel_heart(ctx: &CanvasRenderingContext2d, x: f64, y: f64, scale: f64) {
    let heart = vec![
        vec!["", "#FF0000", "#FF0000", "", "#FF0000", "#FF0000", ""],
        vec!["#FF0000", "#FF69B4", "#FF69B4", "#FF0000", "#FF69B4", "#FF69B4", "#FF0000"],
        vec!["#FF0000", "#FF69B4", "#FF69B4", "#FF69B4", "#FF69B4", "#FF69B4", "#FF0000"],
        vec!["", "#FF0000", "#FF69B4", "#FF69B4", "#FF69B4", "#FF0000", ""],
        vec!["", "", "#FF0000", "#FF69B4", "#FF0000", "", ""],
        vec!["", "", "", "#FF0000", "", "", ""],
    ];
    
    for (row_idx, row) in heart.iter().enumerate() {
        for (col_idx, color) in row.iter().enumerate() {
            if *color != "" {
                ctx.set_fill_style(&JsValue::from_str(color));
                ctx.fill_rect(
                    x + col_idx as f64 * scale,
                    y + row_idx as f64 * scale,
                    scale,
                    scale
                );
            }
        }
    }
}

fn draw_heart_quarters(ctx: &CanvasRenderingContext2d, x: f64, y: f64, quarters: i32) {
    if quarters <= 0 {
        // Empty heart - just outline
        ctx.set_stroke_style(&JsValue::from_str("#800000"));
        ctx.set_line_width(1.5);
        ctx.stroke_rect(x, y, 21.0, 18.0);
        return;
    }
    
    if quarters >= 4 {
        // Full heart
        draw_pixel_heart(ctx, x, y, 3.0);
        return;
    }
    
    // Partial hearts - draw outline then fill partially
    ctx.set_stroke_style(&JsValue::from_str("#800000"));
    ctx.set_line_width(1.5);
    ctx.stroke_rect(x, y, 21.0, 18.0);
    
    ctx.set_fill_style(&JsValue::from_str("#FF0000"));
    match quarters {
        1 => ctx.fill_rect(x + 1.0, y + 1.0, 5.0, 16.0),
        2 => ctx.fill_rect(x + 1.0, y + 1.0, 10.0, 16.0),
        3 => ctx.fill_rect(x + 1.0, y + 1.0, 15.0, 16.0),
        _ => {}
    }
}

#[derive(Clone)]
struct Player {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    vel_x: f64,
    vel_y: f64,
    speed: f64,
    jump_power: f64,
    on_ground: bool,
    facing_right: bool,
    is_attacking: bool,
    attack_cooldown: f64,
    health: f64,  // Changed to f64 for quarter hearts
    max_health: f64,
    animation_frame: f64,
    sword_angle: f64,
    damage_cooldown: f64,
    invincible: bool,
    is_dead: bool,
}

impl Player {
    fn new() -> Self {
        Player {
            x: 100.0,
            y: 300.0,
            width: 24.0,
            height: 32.0,
            vel_x: 0.0,
            vel_y: 0.0,
            speed: 5.0,
            jump_power: 12.0,
            on_ground: false,
            facing_right: true,
            is_attacking: false,
            attack_cooldown: 0.0,
            health: 28.0,  // 7 hearts * 4 quarters = 28 quarter hearts
            max_health: 28.0,
            animation_frame: 0.0,
            sword_angle: 90.0,  // Start vertical
            damage_cooldown: 0.0,
            invincible: false,
            is_dead: false,
        }
    }

    fn update(&mut self, delta: f64, platforms: &[Platform]) {
        self.vel_y += 0.5;
        
        if self.vel_y > 15.0 {
            self.vel_y = 15.0;
        }

        self.x += self.vel_x;
        self.y += self.vel_y;

        self.on_ground = false;
        for platform in platforms {
            if self.check_collision(platform) {
                if self.vel_y > 0.0 && self.y < platform.y {
                    self.y = platform.y - self.height;
                    self.vel_y = 0.0;
                    self.on_ground = true;
                }
            }
        }

        if self.y > 450.0 {
            self.y = 450.0;
            self.vel_y = 0.0;
            self.on_ground = true;
        }

        if self.attack_cooldown > 0.0 {
            self.attack_cooldown -= delta * 0.016;  // Normalize delta
            
            if self.attack_cooldown <= 0.0 {
                self.is_attacking = false;
                self.attack_cooldown = 0.0;
            }
        }
        
        // Update damage cooldown
        if self.damage_cooldown > 0.0 {
            self.damage_cooldown -= delta * 0.016;
            if self.damage_cooldown <= 0.0 {
                self.invincible = false;
            }
        }

        self.vel_x *= 0.85;
        
        if self.vel_x.abs() > 0.1 {
            self.animation_frame += 0.2;
            if self.animation_frame >= 4.0 {
                self.animation_frame = 0.0;
            }
        } else {
            self.animation_frame = 0.0;
        }
    }

    fn check_collision(&self, platform: &Platform) -> bool {
        self.x < platform.x + platform.width &&
        self.x + self.width > platform.x &&
        self.y < platform.y + platform.height &&
        self.y + self.height > platform.y
    }

    fn jump(&mut self) {
        if self.on_ground {
            self.vel_y = -self.jump_power;
        }
    }

    fn move_left(&mut self) {
        self.vel_x = -self.speed;
        self.facing_right = false;
    }

    fn move_right(&mut self) {
        self.vel_x = self.speed;
        self.facing_right = true;
    }

    fn attack(&mut self) {
        if self.attack_cooldown <= 0.0 {
            console_log!("SWORD SWING!");
            self.is_attacking = true;
            self.attack_cooldown = 0.4;
            // Sword starts vertical and will swing
        }
    }
    
    fn take_damage(&mut self) {
        if !self.invincible && self.damage_cooldown <= 0.0 && !self.is_dead {
            self.health -= 1.0;  // 1 quarter heart damage
            self.invincible = true;
            self.damage_cooldown = 1.0;  // 1 second of invincibility
            
            let hearts_remaining = self.health / 4.0;
            console_log!("Player took damage! Hearts: {:.2}", hearts_remaining);
            
            if self.health <= 0.0 {
                console_log!("GAME OVER!");
                self.health = 0.0;
                self.is_dead = true;
            }
        }
    }

    fn get_sword_hitbox(&self) -> (f64, f64, f64, f64) {
        if self.is_attacking {
            let swing_progress = 1.0 - (self.attack_cooldown / 0.4);
            
            // Calculate sword position based on rotation
            let start_angle = 90.0_f64.to_radians();
            let end_angle = if self.facing_right {
                -10.0_f64.to_radians()
            } else {
                190.0_f64.to_radians()
            };
            let current_angle = start_angle + (end_angle - start_angle) * swing_progress;
            
            // Hand position (pivot point)
            let hand_x = if self.facing_right {
                self.x + self.width + 2.0
            } else {
                self.x - 6.0
            };
            let hand_y = self.y + 12.0;
            
            // Sword tip position
            let sword_length = 20.0;
            let tip_x = hand_x + current_angle.cos() * sword_length;
            let tip_y = hand_y - current_angle.sin() * sword_length;
            
            // Create hitbox around sword tip area
            let hitbox_size = 20.0;
            (tip_x - hitbox_size/2.0, tip_y - hitbox_size/2.0, hitbox_size, hitbox_size)
        } else {
            // No hitbox when not attacking
            (0.0, 0.0, 0.0, 0.0)
        }
    }
}

#[derive(Clone)]
struct Enemy {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    vel_x: f64,
    vel_y: f64,
    health: i32,
    patrol_start: f64,
    patrol_end: f64,
    speed: f64,
    is_alive: bool,
    hit_flash: f64,
}

impl Enemy {
    fn new(x: f64, y: f64, patrol_range: f64) -> Self {
        Enemy {
            x,
            y,
            width: 21.0,
            height: 27.0,
            vel_x: 1.5,
            vel_y: 0.0,
            health: 1,
            patrol_start: x - patrol_range / 2.0,
            patrol_end: x + patrol_range / 2.0,
            speed: 1.5,
            is_alive: true,
            hit_flash: 0.0,
        }
    }

    fn update(&mut self, delta: f64, platforms: &[Platform]) {
        if !self.is_alive {
            return;
        }
        
        // Update hit flash
        if self.hit_flash > 0.0 {
            self.hit_flash -= delta * 0.05;
        }

        self.vel_y += 0.5;
        if self.vel_y > 15.0 {
            self.vel_y = 15.0;
        }

        self.x += self.vel_x;
        self.y += self.vel_y;

        if self.x <= self.patrol_start || self.x >= self.patrol_end {
            self.vel_x = -self.vel_x;
        }

        for platform in platforms {
            if self.check_collision(platform) {
                if self.vel_y > 0.0 && self.y < platform.y {
                    self.y = platform.y - self.height;
                    self.vel_y = 0.0;
                }
            }
        }

        if self.y > 450.0 {
            self.y = 450.0;
            self.vel_y = 0.0;
        }
    }

    fn check_collision(&self, platform: &Platform) -> bool {
        self.x < platform.x + platform.width &&
        self.x + self.width > platform.x &&
        self.y < platform.y + platform.height &&
        self.y + self.height > platform.y
    }

    fn take_damage(&mut self, from_right: bool) {
        self.health -= 1;
        self.hit_flash = 1.0;
        
        // Add knockback
        self.vel_x = if from_right { -8.0 } else { 8.0 };
        self.vel_y = -3.0;
        
        if self.health <= 0 {
            self.is_alive = false;
        }
    }
}

#[derive(Clone)]
struct Platform {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(Clone)]
struct HeartPickup {
    x: f64,
    y: f64,
    size: f64,
    collected: bool,
    float_offset: f64,
}

impl Platform {
    fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Platform { x, y, width, height }
    }
}

impl HeartPickup {
    fn new(x: f64, y: f64) -> Self {
        HeartPickup {
            x,
            y,
            size: 20.0,
            collected: false,
            float_offset: 0.0,
        }
    }
    
    fn update(&mut self, time: f64) {
        // Floating animation
        self.float_offset = (time * 0.003).sin() * 5.0;
    }
    
    fn check_collision(&self, player: &Player) -> bool {
        !self.collected &&
        player.x < self.x + self.size &&
        player.x + player.width > self.x &&
        player.y < self.y + self.size &&
        player.y + player.height > self.y
    }
}

struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    platforms: Vec<Platform>,
    hearts: Vec<HeartPickup>,
    camera_x: f64,
    keys: KeyState,
    distance_traveled: f64,
    last_platform_x: f64,
    next_platform_seed: u32,
    game_time: f64,
}

#[derive(Default)]
struct KeyState {
    left: bool,
    right: bool,
    up: bool,
    space: bool,
}

impl Game {
    fn new() -> Self {
        // Start with a few initial platforms
        let mut platforms = vec![
            Platform::new(0.0, 450.0, 200.0, 50.0),  // Starting ground
            Platform::new(200.0, 400.0, 100.0, 20.0),
            Platform::new(350.0, 350.0, 80.0, 20.0),
        ];
        
        let mut enemies = vec![
            Enemy::new(250.0, 350.0, 80.0),
        ];
        
        let mut game = Game {
            player: Player::new(),
            enemies,
            platforms,
            hearts: Vec::new(),
            camera_x: 0.0,
            keys: KeyState::default(),
            distance_traveled: 0.0,
            last_platform_x: 350.0,
            next_platform_seed: 1,
            game_time: 0.0,
        };
        
        // Generate initial platforms
        game.generate_platforms_ahead();
        
        game
    }
    
    fn generate_platforms_ahead(&mut self) {
        // Generate platforms up to 1000 pixels ahead of the last platform
        while self.last_platform_x < self.camera_x + 1200.0 {
            // Use simple pseudo-random for platform generation
            self.next_platform_seed = (self.next_platform_seed * 1103515245 + 12345) & 0x7fffffff;
            
            // Random gap between platforms (50-200 pixels)
            let gap = 50.0 + (self.next_platform_seed % 150) as f64;
            
            // Random platform width (60-150 pixels)
            let width = 60.0 + (self.next_platform_seed % 90) as f64;
            
            // Random height variation
            let height_variation = (self.next_platform_seed % 200) as f64 - 100.0;
            let y = 350.0 + height_variation;
            
            // Create new platform
            let x = self.last_platform_x + gap + width / 2.0;
            self.platforms.push(Platform::new(x, y, width, 20.0));
            
            // Chance to spawn enemy on platform (40% chance)
            if self.next_platform_seed % 100 < 40 {
                // Increase enemy density as player progresses
                let difficulty_multiplier = (self.distance_traveled / 1000.0).min(3.0);
                if self.next_platform_seed % 100 < (40.0 * difficulty_multiplier) as u32 {
                    self.enemies.push(Enemy::new(x + width / 4.0, y - 50.0, width * 0.8));
                }
            }
            
            // Chance to spawn heart pickup (15% chance)
            self.next_platform_seed = (self.next_platform_seed * 1103515245 + 12345) & 0x7fffffff;
            if self.next_platform_seed % 100 < 15 {
                // Place heart above platform
                self.hearts.push(HeartPickup::new(x, y - 40.0));
            }
            
            self.last_platform_x = x + width / 2.0;
        }
    }
    
    fn cleanup_behind(&mut self) {
        // Remove platforms and enemies that are too far behind the camera
        let cleanup_x = self.camera_x - 500.0;
        
        self.platforms.retain(|platform| platform.x + platform.width > cleanup_x);
        self.enemies.retain(|enemy| enemy.x > cleanup_x || !enemy.is_alive);
        self.hearts.retain(|heart| heart.x > cleanup_x && !heart.collected);
    }

    fn update(&mut self, delta: f64) {
        // Don't update if player is dead
        if self.player.is_dead {
            return;
        }
        
        // Update game time
        self.game_time += delta;
        
        if self.keys.left {
            self.player.move_left();
        }
        if self.keys.right {
            self.player.move_right();
        }
        if self.keys.up {
            self.player.jump();
        }
        if self.keys.space {
            self.player.attack();
        }

        self.player.update(delta, &self.platforms);
        
        for enemy in &mut self.enemies {
            enemy.update(delta, &self.platforms);
        }
        
        // Update heart pickups
        for heart in &mut self.hearts {
            heart.update(self.game_time);
            
            // Check if player collects heart
            if heart.check_collision(&self.player) {
                // Heal player (1 full heart = 4 quarter hearts)
                if self.player.health < 28.0 {
                    self.player.health = (self.player.health + 4.0).min(28.0);
                    heart.collected = true;
                    console_log!("Heart collected! Health: {}", self.player.health / 4.0);
                }
            }
        }

        // Check for sword hits during the swing (only in first half of animation)
        if self.player.is_attacking {
            let swing_progress = 1.0 - (self.player.attack_cooldown / 0.4);
            if swing_progress < 0.5 {  // Only hit during the actual swing, not return
                let (sx, sy, sw, sh) = self.player.get_sword_hitbox();
                for enemy in &mut self.enemies {
                    if enemy.is_alive && enemy.hit_flash <= 0.0 {
                        // Check collision with extended hitbox during swing
                        if sx < enemy.x + enemy.width &&
                           sx + sw > enemy.x &&
                           sy < enemy.y + enemy.height &&
                           sy + sh > enemy.y {
                            console_log!("ENEMY DEFEATED!");
                            enemy.take_damage(self.player.facing_right);
                            enemy.hit_flash = 1.0;  // Prevent multiple hits
                        }
                    }
                }
            }
        }

        // Check for player-enemy collision and damage player
        for enemy in &self.enemies {
            if enemy.is_alive &&
               !self.player.invincible &&
               self.player.x < enemy.x + enemy.width &&
               self.player.x + self.player.width > enemy.x &&
               self.player.y < enemy.y + enemy.height &&
               self.player.y + self.player.height > enemy.y {
                // Damage and knockback player
                self.player.take_damage();
                
                // Knockback player away from enemy
                if self.player.x < enemy.x {
                    self.player.vel_x = -8.0;
                } else {
                    self.player.vel_x = 8.0;
                }
                self.player.vel_y = -5.0;
                
                // Update hearts display immediately
                break;  // Only take damage from one enemy at a time
            }
        }

        // Update camera to follow player
        self.camera_x = self.player.x - 400.0;
        if self.camera_x < 0.0 {
            self.camera_x = 0.0;
        }
        
        // Track distance traveled
        if self.player.x > self.distance_traveled {
            self.distance_traveled = self.player.x;
        }
        
        // Generate new platforms ahead and cleanup behind
        self.generate_platforms_ahead();
        self.cleanup_behind();
    }

    fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_image_smoothing_enabled(false);
        ctx.clear_rect(0.0, 0.0, 800.0, 500.0);
        
        // Draw sky gradient
        ctx.set_fill_style(&JsValue::from_str("#87CEEB"));
        ctx.fill_rect(0.0, 0.0, 800.0, 300.0);
        ctx.set_fill_style(&JsValue::from_str("#98D8E8"));
        ctx.fill_rect(0.0, 300.0, 800.0, 200.0);

        ctx.save();
        ctx.translate(-self.camera_x, 0.0).unwrap();

        // Draw platforms with pixel art style
        for platform in &self.platforms {
            draw_pixel_platform(ctx, platform.x, platform.y, platform.width, platform.height);
        }

        // Draw enemies as pixel goblins
        for enemy in &self.enemies {
            if enemy.is_alive {
                // Flash white when hit
                if enemy.hit_flash > 0.0 {
                    ctx.set_global_alpha(0.8);
                    ctx.set_fill_style(&JsValue::from_str("#FFFFFF"));
                    ctx.fill_rect(enemy.x, enemy.y, enemy.width, enemy.height);
                    ctx.set_global_alpha(1.0);
                } else {
                    let goblin = create_goblin_sprite();
                    goblin.draw(ctx, enemy.x, enemy.y, 3.0, enemy.vel_x < 0.0);
                }
            }
        }
        
        // Draw heart pickups
        for heart in &self.hearts {
            if !heart.collected {
                let float_y = heart.y + heart.float_offset;
                
                // Draw glowing effect
                ctx.set_fill_style(&JsValue::from_str("#FF69B4"));
                ctx.set_global_alpha(0.3);
                ctx.begin_path();
                ctx.arc(heart.x + 10.0, float_y + 10.0, 15.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
                ctx.fill();
                ctx.set_global_alpha(1.0);
                
                // Draw the pixel heart sprite
                draw_pixel_heart(ctx, heart.x, float_y, 3.0);
            }
        }


        // Draw player with flashing when invincible
        if self.player.invincible && (self.player.damage_cooldown * 10.0) as i32 % 2 == 0 {
            // Flash effect - skip drawing every other frame
        } else {
            // Draw player as pixel knight
            let frame = self.player.animation_frame as usize;
            let knight = create_knight_sprite(frame);
            knight.draw(ctx, self.player.x, self.player.y, 3.0, !self.player.facing_right);
        }
        
        // Always draw the sword
        let sword_base_x = if self.player.facing_right {
            self.player.x + self.player.width + 2.0
        } else {
            self.player.x - 6.0
        };
        let sword_base_y = self.player.y + 8.0;
        
        // Draw sword based on attack state
        if self.player.is_attacking {
            let swing_progress = 1.0 - (self.player.attack_cooldown / 0.4);
            
            // Sword rotates around the hand position (90 degrees to -10 degrees)
            let start_angle = 90.0_f64.to_radians();  // Start vertical
            let end_angle = if self.player.facing_right {
                -10.0_f64.to_radians()  // End slightly past horizontal
            } else {
                190.0_f64.to_radians()  // Mirror for left facing
            };
            
            // Interpolate the angle
            let current_angle = start_angle + (end_angle - start_angle) * swing_progress;
            
            // Fixed pivot point at player's hand
            let hand_x = if self.player.facing_right {
                sword_base_x + 4.0
            } else {
                sword_base_x - 4.0
            };
            let hand_y = sword_base_y + 4.0;
            
            // Sword extends from hand position
            let sword_length = 20.0;
            let sword_x = hand_x + current_angle.cos() * sword_length;
            let sword_y = hand_y - current_angle.sin() * sword_length;
            
            // Draw motion trail for arc effect
            if swing_progress < 0.6 {
                ctx.set_global_alpha(0.2);
                for i in 1..4 {
                    let trail_progress = (swing_progress - (i as f64 * 0.08)).max(0.0);
                    let trail_angle = start_angle + (end_angle - start_angle) * trail_progress;
                    
                    // Trail positions
                    let tx = hand_x + trail_angle.cos() * sword_length;
                    let ty = hand_y - trail_angle.sin() * sword_length;
                    
                    // Draw simple line for trail
                    ctx.set_stroke_style(&JsValue::from_str("#C0C0C0"));
                    ctx.set_line_width(3.0);
                    ctx.begin_path();
                    ctx.move_to(hand_x, hand_y);
                    ctx.line_to(tx, ty);
                    ctx.stroke();
                }
                ctx.set_global_alpha(1.0);
            }
            
            // Draw sword as a line from hand to tip
            ctx.set_stroke_style(&JsValue::from_str("#C0C0C0"));
            ctx.set_line_width(4.0);
            ctx.begin_path();
            ctx.move_to(hand_x, hand_y);
            ctx.line_to(sword_x, sword_y);
            ctx.stroke();
            
            // Draw sword blade highlight
            ctx.set_stroke_style(&JsValue::from_str("#FFFFFF"));
            ctx.set_line_width(2.0);
            ctx.begin_path();
            ctx.move_to(hand_x, hand_y);
            ctx.line_to(sword_x, sword_y);
            ctx.stroke();
            
            // Draw handle at pivot point
            ctx.set_fill_style(&JsValue::from_str("#8B4513"));
            ctx.fill_rect(hand_x - 3.0, hand_y - 3.0, 6.0, 6.0);
            
            // Add impact effect at peak of swing
            if swing_progress > 0.4 && swing_progress < 0.6 {
                ctx.set_stroke_style(&JsValue::from_str("#FFFF00"));
                ctx.set_global_alpha(0.6);
                ctx.set_line_width(8.0);
                ctx.begin_path();
                ctx.move_to(hand_x, hand_y);
                ctx.line_to(sword_x, sword_y);
                ctx.stroke();
                ctx.set_global_alpha(1.0);
            }
            
        } else {
            // Draw sword at rest (vertical)
            let sword = create_sword_sprite_vertical();
            sword.draw(ctx, sword_base_x, sword_base_y - 8.0, 2.0, !self.player.facing_right);
        }

        ctx.restore();

        // Draw 7 hearts with quarter heart precision
        let total_quarters = self.player.health as i32;
        for i in 0..7 {
            let heart_quarters = (total_quarters - (i * 4)).min(4).max(0);
            draw_heart_quarters(ctx, 10.0 + i as f64 * 22.0, 10.0, heart_quarters);
        }
        
        // Show game over message
        if self.player.is_dead {
            ctx.set_fill_style(&JsValue::from_str("#FF0000"));
            ctx.set_font("48px Arial");
            ctx.fill_text("GAME OVER", 250.0, 250.0).unwrap();
        }
        
        // Show distance traveled
        ctx.set_fill_style(&JsValue::from_str("#FFFFFF"));
        ctx.set_font("16px Arial");
        ctx.fill_text(&format!("Distance: {}m", (self.distance_traveled / 10.0) as i32), 650.0, 30.0).unwrap();
    }
}

#[wasm_bindgen]
pub struct GameEngine {
    game: Rc<RefCell<Game>>,
}

#[wasm_bindgen]
impl GameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_log!("Game engine initialized!");
        GameEngine {
            game: Rc::new(RefCell::new(Game::new())),
        }
    }

    pub fn update(&self, delta: f64) {
        self.game.borrow_mut().update(delta);
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        self.game.borrow().render(ctx);
    }

    pub fn key_down(&self, key: String) {
        let mut game = self.game.borrow_mut();
        match key.as_str() {
            "ArrowLeft" | "a" | "A" => game.keys.left = true,
            "ArrowRight" | "d" | "D" => game.keys.right = true,
            "ArrowUp" | "w" | "W" => game.keys.up = true,
            " " => game.keys.space = true,
            _ => {}
        }
    }

    pub fn key_up(&self, key: String) {
        let mut game = self.game.borrow_mut();
        match key.as_str() {
            "ArrowLeft" | "a" | "A" => game.keys.left = false,
            "ArrowRight" | "d" | "D" => game.keys.right = false,
            "ArrowUp" | "w" | "W" => game.keys.up = false,
            " " => game.keys.space = false,
            _ => {}
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Medieval Platformer WASM Module Loaded!");
}