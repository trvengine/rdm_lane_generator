use std::env;
use std::time::Instant;
use rdm_lane_generator::api::*;

fn print_usage(prog: &str) {
    println!("Usage: {} <command> [options]", prog);
    println!();
    println!("Commands:");
    println!("  generate --count <n> [--start <num>]  Generate N primes with tags");
    println!("  balance --count <n>                   Run N primes and show balance stats only");
    println!("  audit --count <n>                     Show every candidate (prime/composite) up to N");
    println!();
    println!("Examples:");
    println!("  {} generate --count 15", prog);
    println!("  {} balance --count 1000", prog);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = &args[0];

    if args.len() < 4 {
        print_usage(prog);
        return;
    }

    let command = &args[1];
    let count: u64 = if args[2] == "--count" {
        args[3].parse().unwrap_or(10)
    } else {
        10
    };
    
    let mut start_n = 5;
    if args.len() >= 6 && args[4] == "--start" {
        start_n = args[5].parse().unwrap_or(5);
    }

    println!("============================================================");
    println!("  RDM(TM) LANE-BALANCED PRIME GENERATOR -- Uchechukwu RDI Law");
    println!("============================================================");
    println!("RDI Spectral Gap            : Delta = 1 (Total Arithmetical Mixing proven)");
    println!("Transition Matrix T         : [[1/2, 1/2], [1/2, 1/2]] -- uniform mixing");
    println!("Command                     : {} {} primes from {}", command, count, start_n);
    println!("------------------------------------------------------------");

    let start_time = Instant::now();
    let mut generator = RDMLaneGenerator::new(start_n);

    if command == "generate" {
        for i in 1..=count {
            let p = generator.next_prime();
            let lane_str = if p.lane == Lane::LMinus { "L-" } else { "L+" };
            let sign = if p.lane == Lane::LMinus { "-" } else { "+" };
            println!("#{:<3}: {:<8} [{}: k={}, 6({}){}1 = {}]", 
                i, p.value, lane_str, p.k, p.k, sign, p.value);
        }
    } else if command == "balance" {
        for _ in 0..count {
            generator.next_prime();
        }
    } else if command == "audit" {
        let mut n = 0;
        let mut k = generator.current_k;
        while n < count {
            let c1 = 6 * k - 1;
            let p1 = is_prime_lei(c1);
            if p1 { generator.l_minus_count += 1; }
            println!("#{:<3}: {:<8} [L- candidate] -> {}", n+1, c1, if p1 { "PRIME (LEI Survivor)" } else { "COMPOSITE (LEI matched)" });
            n += 1;
            if n >= count { break; }
            
            let c2 = 6 * k + 1;
            let p2 = is_prime_lei(c2);
            if p2 { generator.l_plus_count += 1; }
            println!("#{:<3}: {:<8} [L+ candidate] -> {}", n+1, c2, if p2 { "PRIME (LEI Survivor)" } else { "COMPOSITE (LEI matched)" });
            n += 1;
            k += 1;
        }
        generator.total_count = generator.l_minus_count + generator.l_plus_count;
    } else {
        println!("Unknown command: {}", command);
        return;
    }

    let elapsed = start_time.elapsed();
    let total_primes = generator.total_count;
    
    // Safety check to avoid division by zero if count is 0
    let m_pct = if total_primes > 0 { (generator.l_minus_count as f64 / total_primes as f64) * 100.0 } else { 0.0 };
    let p_pct = if total_primes > 0 { (generator.l_plus_count as f64 / total_primes as f64) * 100.0 } else { 0.0 };

    println!("------------------------------------------------------------");
    println!("Primes Generated            : {}", total_primes);
    println!("L- Count                    : {:<4} ({:.2}%)", generator.l_minus_count, m_pct);
    println!("L+ Count                    : {:<4} ({:.2}%)", generator.l_plus_count, p_pct);
    println!("Balance Ratio               : {:.3} / {:.3}", m_pct / 100.0, p_pct / 100.0);
    println!("RDI Law Prediction          : 0.500 / 0.500 (converges as N->infty)");
    println!();
    println!("Note: Lane-balanced primes are suitable for cryptographic key generation");
    println!("where spectral uniformity is required by the RDI Law (Delta=1).");
    println!("Execution Time              : {:?}", elapsed);
    println!("============================================================");
}
