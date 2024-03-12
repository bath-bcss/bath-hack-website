use bhw_types::models::group::GroupMember;
use yew::prelude::*;

pub fn member_card(member: GroupMember) -> Html {
    html! {
        <div
            key={member.bath_username.clone()}
            class="bg-bcss-100 dark:bg-bcss-800 px-4 py-2 rounded-xl w-full sm:w-auto"
        >
            <p class="text-xs mb-1 text-bcss-700 dark:text-bcss-300">{ "Member" }</p>
            <p class="text-bcss-900 dark:text-bcss-200">
                { "Username: " }
                <code>{ member.bath_username }</code>
            </p>
            if let Some(display_name) = member.display_name {
                <p class="text-bcss-800 dark:text-bcss-200 text-lg font-medium leading-snug">
                    { display_name }
                </p>
            }
        </div>
    }
}
