use raylib::core::math::Vector2;

trait IsEntity {
    fn get_pos(&self) -> Vector2;
    fn set_pos(&mut self, new_pos:Vector2);
    fn update_pos(&mut self, vel:Vector2) {
        self.set_pos(
            self.get_pos() + vel
        );
    }

    fn get_vel(&self) -> Vector2;
    fn set_vel(&mut self, new_vel:Vector2);
    fn accelerate(&mut self, acel:Vector2) {
        self.set_vel(
            self.get_vel() + acel
        );
    }

    fn update(&mut self);
}