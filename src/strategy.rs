/// Trading strategy calculation module
/// Implements PERCENTAGE, FIXED, and ADAPTIVE strategies

use crate::settings::{Config, CopyStrategy};

/// Calculate base order size using the configured strategy
pub fn calculate_base_order_size(
    config: &Config,
    trader_order_size_usd: f64,
    _trader_order_price: f64,
) -> f64 {
    let base_amount = match config.copy_strategy {
        CopyStrategy::Percentage => {
            // PERCENTAGE: trader_order_size_usd × (COPY_SIZE / 100)
            trader_order_size_usd * (config.copy_size / 100.0)
        }
        CopyStrategy::Fixed => {
            // FIXED: Always COPY_SIZE (in USD)
            config.copy_size
        }
        CopyStrategy::Adaptive => {
            // ADAPTIVE: Percentage varies based on trader order size
            let effective_percent = calculate_adaptive_percent(
                config,
                trader_order_size_usd,
            );
            trader_order_size_usd * (effective_percent / 100.0)
        }
    };
    
    base_amount
}

/// Calculate effective percentage for ADAPTIVE strategy (public for sell orders)
pub fn calculate_adaptive_percent_for_display(config: &Config, trader_order_size_usd: f64) -> f64 {
    calculate_adaptive_percent(config, trader_order_size_usd)
}

/// Calculate effective percentage for ADAPTIVE strategy
fn calculate_adaptive_percent(config: &Config, trader_order_size_usd: f64) -> f64 {
    let threshold = config.adaptive_threshold_usd;
    
    if trader_order_size_usd < threshold {
        // Small orders: interpolate from MAX_PERCENT down to COPY_SIZE
        let factor = trader_order_size_usd / threshold;
        // effective_percent = MAX_PERCENT - (MAX_PERCENT - COPY_SIZE) × factor
        config.adaptive_max_percent - (config.adaptive_max_percent - config.copy_size) * factor
    } else {
        // Large orders: interpolate from COPY_SIZE down to MIN_PERCENT
        // factor = min(1, trader_order_size / threshold - 1)
        let factor = ((trader_order_size_usd / threshold) - 1.0).min(1.0);
        // effective_percent = COPY_SIZE - (COPY_SIZE - MIN_PERCENT) × factor
        config.copy_size - (config.copy_size - config.adaptive_min_percent) * factor
    }
}

/// Get tiered multiplier for a given trader order size
/// Returns the multiplier to apply, or None if no tiered multipliers configured
pub fn get_tiered_multiplier(
    tiered_multipliers: &Option<String>,
    trader_order_size_usd: f64,
) -> Option<f64> {
    let tiers_str = tiered_multipliers.as_ref()?;
    
    // Parse format: "1-10:2.0,10-100:1.0,100-500:0.5,500+:0.2"
    for tier in tiers_str.split(',') {
        let tier = tier.trim();
        if tier.is_empty() {
            continue;
        }
        
        // Split on colon
        let parts: Vec<&str> = tier.split(':').collect();
        if parts.len() != 2 {
            continue;
        }
        
        let range = parts[0].trim();
        let multiplier_str = parts[1].trim();
        
        let multiplier: f64 = match multiplier_str.parse() {
            Ok(m) => m,
            Err(_) => continue,
        };
        
        // Parse range
        if range.ends_with('+') {
            // Format: "500+"
            let min_str = range.trim_end_matches('+');
            if let Ok(min) = min_str.parse::<f64>() {
                if trader_order_size_usd >= min {
                    return Some(multiplier);
                }
            }
        } else if range.contains('-') {
            // Format: "1-10"
            let range_parts: Vec<&str> = range.split('-').collect();
            if range_parts.len() == 2 {
                if let (Ok(min), Ok(max)) = (
                    range_parts[0].trim().parse::<f64>(),
                    range_parts[1].trim().parse::<f64>(),
                ) {
                    if trader_order_size_usd >= min && trader_order_size_usd < max {
                        return Some(multiplier);
                    }
                }
            }
        }
    }
    
    None
}

/// Apply all multipliers and limits to get final order size
pub fn calculate_final_order_size(
    config: &Config,
    base_amount: f64,
    trader_order_size_usd: f64,
) -> f64 {
    // Step 1: Apply single multiplier
    let mut amount = base_amount * config.trade_multiplier;
    
    // Step 2: Apply tiered multiplier if configured
    if let Some(tiered_mult) = get_tiered_multiplier(&config.tiered_multipliers, trader_order_size_usd) {
        amount *= tiered_mult;
    }
    
    // Step 3: Apply MAX_ORDER_SIZE_USD cap
    amount = amount.min(config.max_order_size_usd);
    
    // Step 4: Apply MIN_ORDER_SIZE_USD floor
    if amount < config.min_order_size_usd {
        // Return 0 to indicate it should be skipped (or could return min_order_size_usd)
        return 0.0;
    }
    
    amount
}

/// Calculate order size in shares from USD amount
pub fn usd_to_shares(usd_amount: f64, price_per_share: f64) -> f64 {
    let safe_price = price_per_share.max(0.0001);
    usd_amount / safe_price
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{Config, CopyStrategy};
    
    fn create_test_config(strategy: CopyStrategy) -> Config {
        Config {
            private_key: "0".repeat(64),
            funder_address: "0".repeat(40),
            wss_url: "wss://test".to_string(),
            enable_trading: true,
            mock_trading: false,
            copy_strategy: strategy,
            copy_size: 10.0,
            trade_multiplier: 1.0,
            adaptive_min_percent: 5.0,
            adaptive_max_percent: 15.0,
            adaptive_threshold_usd: 500.0,
            tiered_multipliers: None,
            max_order_size_usd: 100.0,
            min_order_size_usd: 1.0,
            max_position_size_usd: None,
            max_daily_volume_usd: None,
            cb_large_trade_shares: 1500.0,
            cb_consecutive_trigger: 2,
            cb_sequence_window_secs: 30,
            cb_min_depth_usd: 200.0,
            cb_trip_duration_secs: 120,
        }
    }
    
    #[test]
    fn test_percentage_strategy() {
        let config = create_test_config(CopyStrategy::Percentage);
        let base = calculate_base_order_size(&config, 100.0, 0.5);
        assert!((base - 10.0).abs() < 0.01, "10% of $100 should be $10");
    }
    
    #[test]
    fn test_fixed_strategy() {
        let mut config = create_test_config(CopyStrategy::Fixed);
        config.copy_size = 50.0;
        let base = calculate_base_order_size(&config, 100.0, 0.5);
        assert!((base - 50.0).abs() < 0.01, "FIXED $50 should always be $50");
    }
    
    #[test]
    fn test_adaptive_strategy_small() {
        let config = create_test_config(CopyStrategy::Adaptive);
        // $50 order (below $500 threshold) should use higher percentage
        let base = calculate_base_order_size(&config, 50.0, 0.5);
        // Should be between 15% and 10% (closer to 15% for small orders)
        assert!(base > 7.0 && base < 7.5, "Small order should use higher %");
    }
    
    #[test]
    fn test_adaptive_strategy_large() {
        let config = create_test_config(CopyStrategy::Adaptive);
        // $1000 order (above $500 threshold) should use lower percentage
        let base = calculate_base_order_size(&config, 1000.0, 0.5);
        // Should be between 10% and 5% (closer to 5% for large orders)
        assert!(base > 50.0 && base < 75.0, "Large order should use lower %");
    }
    
    #[test]
    fn test_tiered_multiplier() {
        let tiers = Some("1-10:2.0,10-100:1.0,100-500:0.5,500+:0.2".to_string());
        
        assert_eq!(get_tiered_multiplier(&tiers, 5.0), Some(2.0));
        assert_eq!(get_tiered_multiplier(&tiers, 50.0), Some(1.0));
        assert_eq!(get_tiered_multiplier(&tiers, 200.0), Some(0.5));
        assert_eq!(get_tiered_multiplier(&tiers, 1000.0), Some(0.2));
    }
    
    #[test]
    fn test_max_order_cap() {
        let mut config = create_test_config(CopyStrategy::Percentage);
        config.max_order_size_usd = 100.0;
        config.copy_size = 50.0; // 50% of $500 = $250, should cap at $100
        
        let base = calculate_base_order_size(&config, 500.0, 0.5);
        let final_size = calculate_final_order_size(&config, base, 500.0);
        assert!((final_size - 100.0).abs() < 0.01, "Should cap at $100");
    }
}

