use slint::SharedString;

slint::include_modules!();

const TAX_RATE: f64 = 0.15;
const OWNER_RATE: f64 = 0.55;
const PROFIT_RATE: f64 = 0.05;
const OPERATING_EXPENSES_RATE: f64 = 0.25;

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string: SharedString| {
        let ui = ui_handle.unwrap();
        let number = string.trim().parse::<f64>().unwrap();
        let tax = number * TAX_RATE;
        let owner = number * OWNER_RATE;
        let profit = number * PROFIT_RATE;
        let operating_expenses = number * OPERATING_EXPENSES_RATE;
        let result: String = format!(
            "Tax: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperating Expenses: {:.2}",
            tax, owner, profit, operating_expenses
        );
        ui.set_results(result.into());
    });

    ui.run()
}
