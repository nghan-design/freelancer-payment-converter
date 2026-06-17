#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Vec,
};

#[derive(Clone)]
#[contracttype]
pub struct Payment {
    pub id: u64,
    pub client_name: String,
    pub amount: i128,
    pub currency: String,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct ConversionRecord {
    pub id: u64,
    pub original_amount: i128,
    pub from_currency: String,
    pub to_currency: String,
    pub converted_amount: i128,
    pub rate: i128,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    PaymentCount,
    ConversionCount,
    Payment(u64),
    Conversion(u64),
}

#[contract]
pub struct FreelancerPaymentConverter;

#[contractimpl]
impl FreelancerPaymentConverter {
    // Thêm khoản thanh toán
    pub fn add_payment(
        env: Env,
        client_name: String,
        amount: i128,
        currency: String,
    ) -> u64 {
        let mut count: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::PaymentCount)
            .unwrap_or(0);

        count += 1;

        let payment = Payment {
            id: count,
            client_name,
            amount,
            currency,
            timestamp: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Payment(count), &payment);

        env.storage()
            .persistent()
            .set(&DataKey::PaymentCount, &count);

        count
    }

    // Lấy một payment
    pub fn get_payment(env: Env, id: u64) -> Payment {
        env.storage()
            .persistent()
            .get(&DataKey::Payment(id))
            .unwrap()
    }

    // Lấy tất cả payment
    pub fn get_all_payments(env: Env) -> Vec<Payment> {
        let count: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::PaymentCount)
            .unwrap_or(0);

        let mut payments = Vec::new(&env);

        let mut i = 1;
        while i <= count {
            let payment: Payment = env
                .storage()
                .persistent()
                .get(&DataKey::Payment(i))
                .unwrap();

            payments.push_back(payment);
            i += 1;
        }

        payments
    }

    // Chuyển đổi tiền tệ
    pub fn convert_currency(
        env: Env,
        amount: i128,
        from_currency: String,
        to_currency: String,
        rate: i128,
    ) -> i128 {
        let converted_amount = amount * rate;

        let mut count: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::ConversionCount)
            .unwrap_or(0);

        count += 1;

        let record = ConversionRecord {
            id: count,
            original_amount: amount,
            from_currency,
            to_currency,
            converted_amount,
            rate,
            timestamp: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Conversion(count), &record);

        env.storage()
            .persistent()
            .set(&DataKey::ConversionCount, &count);

        converted_amount
    }

    // Lấy lịch sử chuyển đổi
    pub fn get_conversion_history(env: Env) -> Vec<ConversionRecord> {
        let count: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::ConversionCount)
            .unwrap_or(0);

        let mut history = Vec::new(&env);

        let mut i = 1;
        while i <= count {
            let record: ConversionRecord = env
                .storage()
                .persistent()
                .get(&DataKey::Conversion(i))
                .unwrap();

            history.push_back(record);
            i += 1;
        }

        history
    }
}