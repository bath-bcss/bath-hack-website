use bhw_types::models::group::GroupMember;
use yew::prelude::*;

pub fn member_card(member: GroupMember) -> Html {
    html! {
    <div key={member.bath_username.clone()} class="bg-bcss-100 px-4 py-2 rounded-xl">
        <p class="text-xs mb-1 text-bcss-700">
            {"Member"}
        </p>

        <p class="text-bcss-900">
            {"Username: "}
            <code>{member.bath_username}</code>
        </p>
        if let Some(display_name) = member.display_name {
        <p class="text-bcss-800 text-lg font-medium leading-snug">
            {display_name}
        </p>
        }
    </div>
    }
}
