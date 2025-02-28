use chrono::{prelude::*, TimeDelta};
use gloo_timers::callback::Interval;
use yew::prelude::*;

use crate::components::flashy_homepage::countdown::countdown_unit::CountdownUnit;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub time_to: DateTime<Utc>,
    pub target_name: String,
    #[prop_or_default]
    pub light: bool,
    #[prop_or_default]
    pub class: Option<Classes>,
}

fn get_time_remaining(until: &DateTime<Utc>) -> TimeDelta {
    until.signed_duration_since(Utc::now())
}

#[function_component(CountdownTimer)]
pub fn countdown_timer(props: &Props) -> Html {
    let time_remaining_handle = use_state_eq(|| get_time_remaining(&props.time_to));
    let time_remaining = *time_remaining_handle;

    use_effect_with(
        (props.time_to, time_remaining_handle),
        move |(time_to, time_remaining_handle)| {
            let time_remaining_handle = time_remaining_handle.clone();
            let time_to = time_to.clone();

            let i = Interval::new(1_000, move || {
                time_remaining_handle.set(get_time_remaining(&time_to));
            });

            || {
                i.cancel();
            }
        },
    );

    let num_secs = time_remaining.num_seconds() % 60;
    let num_mins = (time_remaining.num_seconds() % 3600) / 60;
    let num_hours = (time_remaining.num_seconds() % 86400) / 3600;
    let num_days = (time_remaining.num_seconds()) / 86400;

    let classes = classes!(
        "rounded-xl",
        "inline-block",
        if props.light {
            "bg-bcss-100"
        } else {
            "bg-bcss-300"
        },
        props.class.clone()
    );

    let row_classes = classes!(
        "flex",
        "border-t",
        if props.light {
            "border-bcss-300"
        } else {
            "border-bcss-400"
        }
    );

    html! {
        <div class={classes}>
            <p class="text-bcss-900 text-center my-2">{ props.target_name.to_owned() }</p>
            <div class={row_classes}>
                <CountdownUnit
                    light={props.light}
                    unit_name="days"
                    value={num_days}
                    skip_leading_zeroes=true
                />
                <CountdownUnit light={props.light} unit_name="hours" value={num_hours} />
                <CountdownUnit light={props.light} unit_name="mins" value={num_mins} />
                <CountdownUnit light={props.light} unit_name="secs" value={num_secs} />
            </div>
        </div>
    }
}
