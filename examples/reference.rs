// Reference Example: Bulk Usage of rust_ti API
//
// This file demonstrates the various functions available in the rust_ti crate.
// Each section shows a direct call of a function or method from the library, with minimal setup and no explanatory narrative.
//
// Data is generated up front and reused across calls. Each function is called with representative arguments.
// Output is printed directly for reference and exploration purposes.

use rust_ti;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let open = vec![
        4334.23, 4364.27, 4366.21, 4384.37, 4391.41, 4364.15, 4406.66, 4458.97, 4505.30, 4497.08,
        4509.55, 4511.70, 4538.77, 4553.04, 4555.84, 4554.86, 4545.55, 4571.84, 4554.87, 4559.43,
        4564.37, 4557.25, 4586.23, 4568.84, 4576.20, 4593.39, 4618.30, 4646.20, 4721.04, 4714.23,
        4725.58, 4743.72, 4764.73, 4724.29, 4753.92, 4758.86, 4773.45, 4786.44, 4782.88, 4745.20,
        4725.07, 4697.42, 4690.57, 4703.70, 4741.93, 4759.94, 4792.13, 4791.18, 4772.35, 4739.13,
        4760.10, 4796.28, 4853.42, 4856.80, 4888.56, 4886.66, 4888.91, 4892.95, 4925.89, 4899.19,
        4861.11, 4916.06, 4957.19, 4950.16, 4973.05, 4995.16, 5004.17, 5026.83, 4967.94, 4976.44,
        5003.14, 5031.13, 4989.32, 4963.03, 5038.83, 5100.92, 5093.00, 5074.60, 5067.20, 5085.36,
        5098.51, 5130.99, 5110.52, 5108.03, 5132.38, 5164.46, 5111.96, 5134.30, 5173.49, 5175.14,
        5123.31, 5154.77, 5139.09, 5181.69, 5253.43, 5242.48, 5219.52, 5228.85, 5226.31, 5248.03,
        5257.97, 5204.29, 5194.37, 5244.05, 5158.95, 5211.37, 5217.03, 5167.88, 5172.95, 5171.51,
        5149.67, 5064.59, 5068.97, 5031.52, 5005.44, 4987.33, 5028.85, 5084.86, 5019.88, 5084.65,
        5114.13, 5103.78, 5029.03, 5049.32, 5122.78, 5142.42, 5187.20, 5168.98, 5189.03, 5225.49,
        5233.08, 5221.10, 5263.26, 5310.07, 5303.10, 5305.35, 5298.69, 5319.28, 5340.26, 5281.45,
        5315.91, 5278.73, 5259.77, 5243.21, 5297.15, 5278.24, 5314.48, 5357.80, 5343.81, 5341.22,
        5353.00, 5409.13, 5441.93, 5424.08, 5431.11, 5476.15, 5499.99, 5466.77, 5459.58, 5460.73,
        5460.71, 5473.59, 5488.48, 5471.08, 5461.84, 5507.44, 5537.91, 5572.75, 5584.24, 5591.26,
        5635.21, 5590.76, 5638.16, 5644.09, 5610.07, 5608.56,
    ];

    let high = vec![
        4373.62, 4372.21, 4386.26, 4391.20, 4393.40, 4418.03, 4421.76, 4508.67, 4521.17, 4511.99,
        4520.12, 4557.11, 4542.14, 4568.43, 4560.31, 4560.52, 4568.14, 4587.64, 4569.89, 4599.39,
        4572.37, 4578.56, 4590.74, 4590.92, 4609.23, 4623.71, 4643.93, 4709.69, 4738.57, 4725.53,
        4749.52, 4768.69, 4778.01, 4748.71, 4772.94, 4784.72, 4785.39, 4793.30, 4788.43, 4754.33,
        4729.29, 4726.78, 4721.49, 4764.54, 4765.47, 4790.80, 4798.50, 4802.40, 4782.34, 4744.23,
        4785.79, 4842.07, 4868.41, 4866.48, 4903.68, 4898.15, 4906.69, 4929.31, 4931.09, 4906.75,
        4906.97, 4975.29, 4957.19, 4957.77, 4999.89, 5000.40, 5030.06, 5048.39, 4971.30, 5002.52,
        5032.72, 5038.70, 4993.71, 4983.21, 5094.39, 5111.06, 5097.66, 5080.69, 5077.37, 5104.99,
        5140.33, 5149.67, 5114.54, 5127.97, 5165.62, 5189.26, 5124.66, 5179.87, 5179.14, 5176.85,
        5136.86, 5175.60, 5180.31, 5226.19, 5261.10, 5246.09, 5229.09, 5235.16, 5249.26, 5264.85,
        5263.95, 5208.34, 5228.75, 5256.59, 5222.18, 5219.57, 5224.81, 5178.43, 5211.78, 5175.03,
        5168.43, 5079.84, 5077.96, 5056.66, 5019.02, 5038.84, 5076.12, 5089.48, 5057.75, 5114.62,
        5123.49, 5110.83, 5096.12, 5073.21, 5139.12, 5181.00, 5200.23, 5191.95, 5215.30, 5239.66,
        5237.26, 5250.37, 5311.76, 5325.49, 5305.45, 5325.32, 5324.32, 5323.18, 5341.88, 5311.65,
        5315.91, 5282.27, 5260.21, 5280.33, 5302.11, 5298.80, 5354.16, 5362.35, 5375.08, 5365.79,
        5375.95, 5447.25, 5441.93, 5432.39, 5488.50, 5490.38, 5505.53, 5478.31, 5490.66, 5472.88,
        5483.14, 5490.81, 5523.64, 5479.55, 5509.69, 5539.27, 5570.33, 5583.11, 5590.75, 5635.39,
        5642.45, 5655.56, 5666.94, 5669.67, 5622.49, 5614.05,
    ];

    let low = vec![
        4334.23, 4347.53, 4355.41, 4359.76, 4343.94, 4353.34, 4393.82, 4458.97, 4495.31, 4487.83,
        4499.66, 4510.36, 4525.51, 4545.05, 4552.80, 4546.32, 4540.51, 4547.15, 4537.24, 4554.71,
        4546.72, 4551.68, 4546.50, 4565.22, 4574.06, 4593.39, 4608.09, 4643.23, 4694.34, 4704.69,
        4725.58, 4743.72, 4697.82, 4708.35, 4736.77, 4758.45, 4768.90, 4780.98, 4751.99, 4722.67,
        4699.71, 4687.53, 4682.11, 4699.82, 4730.35, 4756.20, 4739.58, 4768.98, 4747.12, 4714.82,
        4740.57, 4785.87, 4844.05, 4844.37, 4865.94, 4869.34, 4881.47, 4887.40, 4916.27, 4845.15,
        4853.52, 4907.99, 4918.09, 4934.88, 4969.05, 4987.09, 5000.34, 5016.83, 4920.31, 4956.45,
        4999.44, 4999.52, 4955.02, 4946.00, 5038.83, 5081.46, 5068.91, 5057.29, 5058.35, 5061.89,
        5094.16, 5127.18, 5056.82, 5092.22, 5128.21, 5117.50, 5091.14, 5114.48, 5151.88, 5123.30,
        5104.35, 5145.47, 5131.59, 5171.55, 5240.66, 5229.87, 5216.09, 5203.42, 5213.92, 5245.82,
        5229.20, 5184.05, 5194.37, 5146.06, 5157.21, 5197.35, 5160.78, 5138.70, 5138.77, 5107.94,
        5052.47, 5039.83, 5007.25, 5001.89, 4953.56, 4969.40, 5027.96, 5047.02, 4990.58, 5073.14,
        5088.65, 5035.31, 5013.45, 5011.05, 5101.22, 5142.42, 5178.96, 5165.86, 5180.41, 5209.68,
        5211.16, 5217.98, 5263.26, 5296.19, 5283.59, 5302.40, 5297.87, 5286.01, 5256.93, 5278.39,
        5280.89, 5262.70, 5222.10, 5191.68, 5234.32, 5257.63, 5297.64, 5335.36, 5331.33, 5331.52,
        5327.25, 5409.13, 5402.51, 5403.75, 5420.40, 5471.32, 5455.56, 5452.03, 5447.59, 5446.56,
        5451.87, 5467.54, 5451.12, 5446.53, 5458.43, 5507.42, 5531.63, 5562.51, 5574.57, 5586.44,
        5576.53, 5590.44, 5614.75, 5639.02, 5584.81, 5522.81,
    ];

    let close = vec![
        4358.34, 4365.98, 4378.38, 4382.78, 4347.35, 4415.24, 4411.55, 4495.70, 4502.88, 4508.24,
        4514.02, 4547.38, 4538.19, 4556.62, 4559.34, 4550.43, 4554.89, 4550.58, 4567.80, 4594.63,
        4569.78, 4567.18, 4549.34, 4585.59, 4604.37, 4622.44, 4643.70, 4707.09, 4719.55, 4719.19,
        4740.56, 4768.37, 4698.35, 4746.75, 4754.63, 4774.75, 4781.58, 4783.35, 4769.83, 4742.83,
        4704.81, 4688.68, 4697.24, 4763.54, 4756.50, 4783.45, 4780.24, 4783.83, 4765.98, 4739.21,
        4780.94, 4839.81, 4850.43, 4864.60, 4868.55, 4894.16, 4890.97, 4927.93, 4924.97, 4845.65,
        4906.19, 4958.61, 4942.81, 4954.23, 4995.06, 4997.91, 5026.61, 5021.84, 4953.17, 5000.62,
        5029.73, 5005.57, 4975.51, 4981.80, 5087.03, 5088.80, 5069.53, 5078.18, 5069.76, 5096.27,
        5137.08, 5130.95, 5078.65, 5104.76, 5157.36, 5123.69, 5117.94, 5175.27, 5165.31, 5150.48,
        5117.09, 5149.42, 5178.51, 5224.62, 5241.53, 5234.18, 5218.19, 5203.58, 5248.49, 5254.35,
        5243.77, 5205.81, 5211.49, 5147.21, 5204.34, 5202.39, 5209.91, 5160.64, 5199.06, 5123.41,
        5061.82, 5051.41, 5022.21, 5011.12, 4967.23, 5010.60, 5070.55, 5071.63, 5048.42, 5099.96,
        5116.17, 5035.69, 5018.39, 5064.20, 5127.79, 5180.74, 5187.70, 5187.67, 5214.08, 5222.68,
        5221.42, 5246.68, 5308.15, 5297.10, 5303.27, 5308.13, 5321.41, 5307.01, 5267.84, 5304.72,
        5306.04, 5266.95, 5235.48, 5277.51, 5283.40, 5291.34, 5354.03, 5352.96, 5346.99, 5360.79,
        5375.32, 5421.03, 5433.74, 5431.60, 5473.23, 5487.03, 5473.17, 5464.62, 5447.87, 5469.30,
        5477.90, 5482.87, 5460.48, 5475.09, 5509.01, 5537.02, 5567.19, 5572.85, 5576.98, 5633.91,
        5584.54, 5615.35, 5631.22, 5667.20, 5588.27, 5544.59,
    ];

    let volume = vec![
        4570960000.0,
        3656340000.0,
        3791230000.0,
        3729510000.0,
        3900780000.0,
        3665080000.0,
        3326240000.0,
        4700350000.0,
        4347170000.0,
        3964520000.0,
        3777240000.0,
        3644790000.0,
        3511080000.0,
        3042810000.0,
        1639500000.0,
        3403990000.0,
        3586240000.0,
        4418760000.0,
        5399300000.0,
        4397120000.0,
        4369910000.0,
        3909950000.0,
        4245680000.0,
        3818880000.0,
        3707010000.0,
        3823210000.0,
        3808380000.0,
        5063650000.0,
        6314040000.0,
        8218980000.0,
        4060340000.0,
        4026970000.0,
        4201320000.0,
        3431180000.0,
        3046770000.0,
        2513910000.0,
        2748450000.0,
        2698860000.0,
        3126060000.0,
        3743050000.0,
        3950760000.0,
        3715480000.0,
        3844370000.0,
        3742320000.0,
        3529960000.0,
        3498680000.0,
        3759890000.0,
        3486340000.0,
        4260550000.0,
        3928600000.0,
        4019000000.0,
        4287200000.0,
        4297610000.0,
        3912800000.0,
        4330030000.0,
        4020430000.0,
        3353400000.0,
        3525160000.0,
        3836130000.0,
        4696120000.0,
        4386090000.0,
        3974350000.0,
        4023640000.0,
        4440880000.0,
        4895590000.0,
        4341860000.0,
        3912990000.0,
        3805740000.0,
        4302190000.0,
        3845600000.0,
        4137970000.0,
        3833270000.0,
        4034880000.0,
        3788390000.0,
        4051710000.0,
        3672790000.0,
        3683930000.0,
        3925950000.0,
        3789370000.0,
        5219740000.0,
        4748110000.0,
        4758440000.0,
        4418410000.0,
        4559050000.0,
        4137980000.0,
        4208870000.0,
        3896430000.0,
        4080510000.0,
        4282890000.0,
        4687970000.0,
        7753670000.0,
        4036220000.0,
        4031760000.0,
        4064850000.0,
        4207730000.0,
        3374700000.0,
        3331360000.0,
        3871790000.0,
        3850500000.0,
        3998270000.0,
        3325930000.0,
        3886590000.0,
        3703250000.0,
        4075680000.0,
        3386780000.0,
        3278180000.0,
        3400680000.0,
        3845930000.0,
        3509380000.0,
        3963220000.0,
        3950210000.0,
        4006200000.0,
        3596130000.0,
        3619760000.0,
        3878750000.0,
        3820250000.0,
        3751400000.0,
        3656740000.0,
        3958050000.0,
        3604140000.0,
        3447450000.0,
        4082470000.0,
        4544170000.0,
        4381660000.0,
        3924990000.0,
        3683250000.0,
        3987890000.0,
        3842100000.0,
        3727370000.0,
        3617900000.0,
        4255710000.0,
        4763580000.0,
        4360810000.0,
        3817470000.0,
        3578120000.0,
        3420100000.0,
        3662240000.0,
        3847130000.0,
        3869520000.0,
        3005510000.0,
        3751540000.0,
        3552750000.0,
        3818750000.0,
        5437160000.0,
        4046920000.0,
        3707900000.0,
        3591460000.0,
        3609990000.0,
        3692760000.0,
        3622280000.0,
        3568030000.0,
        3962840000.0,
        3530380000.0,
        3438650000.0,
        3447840000.0,
        3544330000.0,
        3847060000.0,
        6773800000.0,
        3696750000.0,
        3591960000.0,
        3563920000.0,
        3589530000.0,
        7199220000.0,
        3488760000.0,
        3329950000.0,
        2179470000.0,
        3253080000.0,
        3185670000.0,
        3232920000.0,
        3336100000.0,
        4020950000.0,
        3700280000.0,
        3620470000.0,
        4041760000.0,
        4246450000.0,
        2926006284.0,
    ];

    let length = close.len();

    // Derived data
    let mut typical_price = Vec::with_capacity(length);
    for i in 0..length {
        typical_price.push((close[i] + high[i] + low[i]) / 3.0);
    }

    // Constants and available models
    let numerator = 5.0;
    let denominator = 4.0;
    let period: usize = 5;
    let long_period: usize = 20;
    let previous_mcginley_dynamic = 0.0;

    let available_models = vec![
        rust_ti::ConstantModelType::SimpleMovingAverage,
        rust_ti::ConstantModelType::SmoothedMovingAverage,
        rust_ti::ConstantModelType::ExponentialMovingAverage,
        rust_ti::ConstantModelType::PersonalisedMovingAverage {
            alpha_num: numerator,
            alpha_den: denominator,
        },
        rust_ti::ConstantModelType::SimpleMovingMedian,
        rust_ti::ConstantModelType::SimpleMovingMode,
    ];
    let available_deviations = vec![
        rust_ti::DeviationModel::StandardDeviation,
        rust_ti::DeviationModel::MeanAbsoluteDeviation,
        rust_ti::DeviationModel::MedianAbsoluteDeviation,
        rust_ti::DeviationModel::ModeAbsoluteDeviation,
        rust_ti::DeviationModel::UlcerIndex,
    ];
    let available_moving_averages = vec![
        rust_ti::MovingAverageType::Simple,
        rust_ti::MovingAverageType::Smoothed,
        rust_ti::MovingAverageType::Exponential,
        rust_ti::MovingAverageType::Personalised {
            alpha_num: numerator,
            alpha_den: denominator,
        },
    ];

    // Basic indicators
    let median = rust_ti::basic_indicators::bulk::median(&typical_price, period);
    println!("Median: {:?}", median);

    let mode = rust_ti::basic_indicators::bulk::mode(&typical_price, period);
    println!("Mode: {:?}", mode);

    let log = rust_ti::basic_indicators::bulk::log(&typical_price);
    println!("Log: {:?}", log);

    let log_diff = rust_ti::basic_indicators::bulk::log_difference(&typical_price);
    println!("Log difference: {:?}", log_diff);

    let variance = rust_ti::basic_indicators::bulk::variance(&typical_price, period);
    println!("Variance: {:?}", variance);

    let standard_dev = rust_ti::basic_indicators::bulk::standard_deviation(&typical_price, period);
    println!("Standard Deviation: {:?}", standard_dev);

    let mean_ad = rust_ti::basic_indicators::bulk::absolute_deviation(
        &typical_price,
        period,
        rust_ti::CentralPoint::Mean,
    );
    println!("Mean Absolute Deviation: {:?}", mean_ad);

    let median_ad = rust_ti::basic_indicators::bulk::absolute_deviation(
        &typical_price,
        period,
        rust_ti::CentralPoint::Median,
    );
    println!("Median Absolute Deviation: {:?}", median_ad);

    let mode_ad = rust_ti::basic_indicators::bulk::absolute_deviation(
        &typical_price,
        period,
        rust_ti::CentralPoint::Mode,
    );
    println!("Mode Absolute Deviation: {:?}", mode_ad);

    // Candle indicators
    let difference = 2.0;
    for model in &available_models {
        let envelope = rust_ti::candle_indicators::bulk::moving_constant_envelopes(
            &typical_price,
            *model,
            difference,
            period,
        );
        println!("{:?} Envelope: {:?}", model, envelope);
    }

    let mcginley_envelope = rust_ti::candle_indicators::bulk::mcginley_dynamic_envelopes(
        &typical_price,
        difference,
        previous_mcginley_dynamic,
        period,
    );
    println!("McGinley Envelope: {:?}", mcginley_envelope);

    let deviation_multiplier = 2.0;
    for model in &available_models {
        for deviation in &available_deviations {
            let bands = rust_ti::candle_indicators::bulk::moving_constant_bands(
                &typical_price,
                *model,
                *deviation,
                deviation_multiplier,
                period,
            );
            println!("{:?} Band, {:?}: {:?}", model, deviation, bands);
        }
    }

    for deviation in &available_deviations {
        let md_bands = rust_ti::candle_indicators::bulk::mcginley_dynamic_bands(
            &typical_price,
            *deviation,
            deviation_multiplier,
            previous_mcginley_dynamic,
            period,
        );
        println!("McGinley Dynamic Band {:?}: {:?}", deviation, md_bands);
    }

    let ichimoku_cloud =
        rust_ti::candle_indicators::bulk::ichimoku_cloud(&high, &low, &close, 9, 26, 52);
    println!("Ichimoku cloud: {:?}", ichimoku_cloud);

    let donchian_channels =
        rust_ti::candle_indicators::bulk::donchian_channels(&high, &low, period);
    println!("Donchian Channels: {:?}", donchian_channels);

    let multiplier = 2.0;
    for model in &available_models {
        for atr_model in &available_models {
            let keltner_channel = rust_ti::candle_indicators::bulk::keltner_channel(
                &high[1..],
                &low[1..],
                &close[..length - 1],
                *model,
                *atr_model,
                multiplier,
                period,
            );
            println!(
                "Keltner Channel {:?} {:?}: {:?}",
                model, atr_model, keltner_channel
            );
        }
    }

    for model in &available_models {
        let supertrend = rust_ti::candle_indicators::bulk::supertrend(
            &high, &low, &close, *model, multiplier, period,
        );
        println!("Supertrend {:?}: {:?}", model, supertrend);
    }

    // Momentum indicators
    for model in &available_models {
        let rsi = rust_ti::momentum_indicators::bulk::relative_strength_index(
            &typical_price,
            *model,
            period,
        );
        println!("{:?} RSI: {:?}", model, rsi);
    }

    let stochastic_oscillator =
        rust_ti::momentum_indicators::bulk::stochastic_oscillator(&typical_price, period);
    println!("Stochastic Oscillator: {:?}", stochastic_oscillator);

    for model in &available_models {
        let slow_so = rust_ti::momentum_indicators::bulk::slow_stochastic(
            &stochastic_oscillator,
            *model,
            period,
        );
        println!("Slow Stochastic {:?}: {:?}", model, slow_so);

        for slowest_model in &available_models {
            let slowest_so = rust_ti::momentum_indicators::bulk::slowest_stochastic(
                &slow_so,
                *slowest_model,
                period,
            );
            println!("Slowest Stochastic {:?}: {:?}", slowest_model, slowest_so);
        }
    }

    let mfi = rust_ti::momentum_indicators::bulk::money_flow_index(&typical_price, &volume, period);
    println!("Money Flow Index: {:?}", mfi);

    let roc = rust_ti::momentum_indicators::bulk::rate_of_change(&typical_price);
    println!("Rate of Change: {:?}", roc);

    let previous_obv = 0.0;
    let obv = rust_ti::momentum_indicators::bulk::on_balance_volume(
        &typical_price,
        &volume,
        previous_obv,
    );
    println!("On Balance Volume: {:?}", obv);

    let constant_multiplier = 0.015;
    for model in &available_models {
        for deviation_model in &available_deviations {
            let cci = rust_ti::momentum_indicators::bulk::commodity_channel_index(
                &typical_price,
                *model,
                *deviation_model,
                constant_multiplier,
                period,
            );
            println!("CCI {:?} {:?}: {:?}", model, deviation_model, cci);
        }
    }

    for deviation_model in &available_deviations {
        let md_cci = rust_ti::momentum_indicators::bulk::mcginley_dynamic_commodity_channel_index(
            &typical_price,
            previous_mcginley_dynamic,
            *deviation_model,
            constant_multiplier,
            period,
        );
        println!("McGinley Dynamic CCI {:?}: {:?}", deviation_model, md_cci);
    }

    for short_model in &available_models {
        for long_model in &available_models {
            let macd = rust_ti::momentum_indicators::bulk::macd_line(
                &typical_price,
                period,
                *short_model,
                long_period,
                *long_model,
            );
            println!("MACD {:?} {:?}: {:?}", short_model, long_model, macd);

            for signal_model in &available_models {
                let signal =
                    rust_ti::momentum_indicators::bulk::signal_line(&macd, *signal_model, period);
                println!("Signal line {:?}: {:?}", signal_model, signal);
            }
        }
    }

    let md_macd = rust_ti::momentum_indicators::bulk::mcginley_dynamic_macd_line(
        &typical_price,
        period,
        previous_mcginley_dynamic,
        long_period,
        previous_mcginley_dynamic,
    );
    println!("McGinley Dynamic MACD: {:?}", md_macd);

    let previous_ad = 0.0;
    for short_model in &available_models {
        for long_model in &available_models {
            let co = rust_ti::momentum_indicators::bulk::chaikin_oscillator(
                &high,
                &low,
                &close,
                &volume,
                period,
                long_period,
                previous_ad,
                *short_model,
                *long_model,
            );
            println!(
                "Chaikin Oscillator {:?} {:?}: {:?}",
                short_model, long_model, co
            );
        }
    }

    for model in &available_models {
        let ppo = rust_ti::momentum_indicators::bulk::percentage_price_oscillator(
            &typical_price,
            period,
            long_period,
            *model,
        );
        println!("PPO {:?}: {:?}", model, ppo);
        for moving_average in &available_moving_averages {
            let signal =
                rust_ti::moving_average::bulk::moving_average(&ppo, *moving_average, period);
            println!("{:?} Signal Line: {:?}", moving_average, signal);
        }
    }

    let cmo =
        rust_ti::momentum_indicators::bulk::chande_momentum_oscillator(&typical_price, period);
    println!("Chande Momentum Oscillator: {:?}", cmo);

    // Moving Averages
    for moving_average in &available_moving_averages {
        let ma =
            rust_ti::moving_average::bulk::moving_average(&typical_price, *moving_average, period);
        println!("{:?} Moving Average: {:?}", moving_average, ma);
    }

    let mcginley_dynamic =
        rust_ti::moving_average::bulk::mcginley_dynamic(&typical_price, 0.0, period);
    println!("McGinley Dynamic: {:?}", mcginley_dynamic);

    // Other indicators
    let true_range = rust_ti::other_indicators::bulk::true_range(&close, &high, &low);
    println!("True Range: {:?}", true_range);

    for model in &available_models {
        let average_true_range = rust_ti::other_indicators::bulk::average_true_range(
            &close, &high, &low, *model, period,
        );
        println!("{:?} Average True Range: {:?}", model, average_true_range);
    }

    let ibs = rust_ti::other_indicators::bulk::internal_bar_strength(&high, &low, &close);
    println!("Internal Bar Strength: {:?}", ibs);

    for model in &available_models {
        let pi = rust_ti::other_indicators::bulk::positivity_indicator(
            &open[1..],
            &close[..length - 1],
            period,
            *model,
        );
        println!("{:?} Positivity Index: {:?}", model, pi);
    }

    // Strength indicators
    let ad = rust_ti::strength_indicators::bulk::accumulation_distribution(
        &high, &low, &close, &volume, 0.0,
    );
    println!("Accumulation Distribution: {:?}", ad);

    let pvi = rust_ti::strength_indicators::bulk::positive_volume_index(&close, &volume, 0.0);
    println!("Positive Volume Index: {:?}", pvi);

    let nvi = rust_ti::strength_indicators::bulk::negative_volume_index(&close, &volume, 0.0);
    println!("Negative Volume Index: {:?}", nvi);

    for model in &available_models {
        let rvi = rust_ti::strength_indicators::bulk::relative_vigor_index(
            &open, &high, &low, &close, *model, period,
        );
        println!("{:?} Relative Vigor Index: {:?}", model, rvi);
        for signal_model in &available_moving_averages {
            let signal = rust_ti::moving_average::bulk::moving_average(&rvi, *signal_model, period);
            println!("{:?} Signal Line: {:?}", signal_model, signal);
        }
    }

    // Trend indicators
    let aroon_indicator = rust_ti::trend_indicators::bulk::aroon_indicator(&high, &low, period);
    println!("Aroon Indicator: {:?}", aroon_indicator);

    let ptps = rust_ti::trend_indicators::bulk::parabolic_time_price_system(
        &high,
        &low,
        0.02,
        0.2,
        0.02,
        rust_ti::Position::Long,
        0.0,
    );
    println!("Parabolic Time Price System: {:?}", ptps);

    for model in &available_models {
        let dms = rust_ti::trend_indicators::bulk::directional_movement_system(
            &high, &low, &close, period, *model,
        );
        println!("{:?} Directional Movement System: {:?}", model, dms);
    }

    let vpt =
        rust_ti::trend_indicators::bulk::volume_price_trend(&typical_price, &volume[1..], 0.0);
    println!("Volume Price Trend: {:?}", vpt);

    for first_model in &available_models {
        for second_model in &available_models {
            let tsi = rust_ti::trend_indicators::bulk::true_strength_index(
                &typical_price,
                *first_model,
                period,
                *second_model,
                period,
            );
            println!(
                "{:?} {:?} True Strength Index: {:?}",
                first_model, second_model, tsi
            );
            for signal_model in &available_moving_averages {
                let signal =
                    rust_ti::moving_average::bulk::moving_average(&tsi, *signal_model, period);
                println!("{:?} Signal Line: {:?}", signal_model, signal);
            }
        }
    }

    // Volatility indicators
    let ulcer_index = rust_ti::volatility_indicators::bulk::ulcer_index(&typical_price, period);
    println!("Ulcer Index: {:?}", ulcer_index);

    let constant_multiplier = 3.0;
    for model in &available_models {
        let vs = rust_ti::volatility_indicators::bulk::volatility_system(
            &high,
            &low,
            &close,
            period,
            constant_multiplier,
            *model,
        );
        println!("{:?} Volatility System: {:?}", model, vs);
    }

    // Chart trends (truncated, see documentation for argument meaning)
    let trend_break_config = rust_ti::chart_trends::TrendBreakConfig::default();
    let break_down_trends = rust_ti::chart_trends::break_down_trends(
        &close, trend_break_config);
    println!("Broken down trends: {:?}", break_down_trends);

    let valleys = rust_ti::chart_trends::valleys(&close, 30, 5);
    println!("Valleys: {:?}", valleys);

    let peaks = rust_ti::chart_trends::peaks(&close, 30, 5);
    println!("Peaks: {:?}", peaks);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
