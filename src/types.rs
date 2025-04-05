pub struct SnipeConfig {
    pub sol_amount: f32,
    pub slippage: u8,
    pub priority_fee: f32,
}

impl Default for SnipeConfig {
    fn default() -> Self {
        SnipeConfig {
            sol_amount: 1.0,
            slippage: 15,
            priority_fee: 0.1,
        }
    }
}

impl SnipeConfig {
    pub fn set_sol_amount(&mut self, value: f32) {
        self.sol_amount = value;
    }

    pub fn set_slippage(&mut self, value: u8) {
        self.slippage = value;
    }

    pub fn set_priority_fee(&mut self, value: f32) {
        self.priority_fee = value;
    }
}

pub struct SnipeTarget {
    pub target_name: String,
    pub snipe_config: SnipeConfig,
    pub is_active: bool,
    pub deactivate_on_snipe: bool,
}

impl SnipeTarget {
    pub fn set_name(&mut self, value: String) {
        self.target_name = value;
    }

    pub fn set_snipe_config(&mut self, value: SnipeConfig) {
        self.snipe_config = value;
    }

    pub fn set_is_active(&mut self, value: bool) {
        self.is_active = value;
    }
    pub fn set_deactivate_on_snipe(&mut self) {
        if self.is_active {
            self.is_active = false
        };
    }
}

impl Default for SnipeTarget {
    fn default() -> Self {
        SnipeTarget {
            target_name: String::from("None"),
            snipe_config: SnipeConfig::default(),
            is_active: false,
            deactivate_on_snipe: true,
        }
    }
}
