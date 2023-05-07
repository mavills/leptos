#[cfg(not(any(feature = "csr", feature = "hydrate")))]
#[test]
fn simple_ssr_test() {
    use leptos::*;

    _ = create_scope(create_runtime(), |cx| {
        let (value, set_value) = create_signal( 0);
        let rendered = view! {
            
            <div>
                <button on:click=move |_| set_value.update(|value| *value -= 1)>"-1"</button>
                <span>"Value: " {move || value.get().to_string()} "!"</span>
                <button on:click=move |_| set_value.update(|value| *value += 1)>"+1"</button>
            </div>
        };

        assert_eq!(
            rendered.into_view().render_to_string(),
            "<!--leptos-view|leptos-tests-ssr.rs-8|open--><div \
             id=\"_0-1\"><button id=\"_0-2\">-1</button><span \
             id=\"_0-3\">Value: \
             <!--hk=_0-4o|leptos-dyn-child-start-->0<!\
             --hk=_0-4c|leptos-dyn-child-end-->!</span><button \
             id=\"_0-5\">+1</button></div><!--leptos-view|leptos-tests-ssr.\
             rs-8|close-->"
        );
    });
}

#[cfg(not(any(feature = "csr", feature = "hydrate")))]
#[test]
fn ssr_test_with_components() {
    use leptos::*;

    #[component]
    fn Counter( initial_value: i32) -> impl IntoView {
        let (value, set_value) = create_signal( initial_value);
        view! {
            
            <div>
                <button on:click=move |_| set_value.update(|value| *value -= 1)>"-1"</button>
                <span>"Value: " {move || value.get().to_string()} "!"</span>
                <button on:click=move |_| set_value.update(|value| *value += 1)>"+1"</button>
            </div>
        }
    }

    _ = create_scope(create_runtime(), |cx| {
        let rendered = view! {
            
            <div class="counters">
                <Counter initial_value=1/>
                <Counter initial_value=2/>
            </div>
        };

        assert_eq!(
            rendered.into_view().render_to_string(),
            "<!--leptos-view|leptos-tests-ssr.rs-49|open--><div id=\"_0-1\" \
             class=\"counters\"><!--hk=_0-1-0o|leptos-counter-start--><!\
             --leptos-view|leptos-tests-ssr.rs-38|open--><div \
             id=\"_0-1-1\"><button id=\"_0-1-2\">-1</button><span \
             id=\"_0-1-3\">Value: \
             <!--hk=_0-1-4o|leptos-dyn-child-start-->1<!\
             --hk=_0-1-4c|leptos-dyn-child-end-->!</span><button \
             id=\"_0-1-5\">+1</button></div><!--leptos-view|leptos-tests-ssr.\
             rs-38|close--><!--hk=_0-1-0c|leptos-counter-end--><!\
             --hk=_0-1-5-0o|leptos-counter-start--><!\
             --leptos-view|leptos-tests-ssr.rs-38|open--><div \
             id=\"_0-1-5-1\"><button id=\"_0-1-5-2\">-1</button><span \
             id=\"_0-1-5-3\">Value: \
             <!--hk=_0-1-5-4o|leptos-dyn-child-start-->2<!\
             --hk=_0-1-5-4c|leptos-dyn-child-end-->!</span><button \
             id=\"_0-1-5-5\">+1</button></div><!\
             --leptos-view|leptos-tests-ssr.rs-38|close--><!\
             --hk=_0-1-5-0c|leptos-counter-end--></div><!\
             --leptos-view|leptos-tests-ssr.rs-49|close-->"
        );
    });
}

#[cfg(not(any(feature = "csr", feature = "hydrate")))]
#[test]
fn ssr_test_with_snake_case_components() {
    use leptos::*;

    #[component]
    fn snake_case_counter( initial_value: i32) -> impl IntoView {
        let (value, set_value) = create_signal( initial_value);
        view! {
            
            <div>
                <button on:click=move |_| set_value.update(|value| *value -= 1)>"-1"</button>
                <span>"Value: " {move || value.get().to_string()} "!"</span>
                <button on:click=move |_| set_value.update(|value| *value += 1)>"+1"</button>
            </div>
        }
    }

    _ = create_scope(create_runtime(), |cx| {
        let rendered = view! {
            
            <div class="counters">
                <SnakeCaseCounter initial_value=1/>
                <SnakeCaseCounter initial_value=2/>
            </div>
        };

        assert_eq!(
            rendered.into_view().render_to_string(),
            "<!--leptos-view|leptos-tests-ssr.rs-101|open--><div id=\"_0-1\" \
             class=\"counters\"><!\
             --hk=_0-1-0o|leptos-snake-case-counter-start--><!\
             --leptos-view|leptos-tests-ssr.rs-90|open--><div \
             id=\"_0-1-1\"><button id=\"_0-1-2\">-1</button><span \
             id=\"_0-1-3\">Value: \
             <!--hk=_0-1-4o|leptos-dyn-child-start-->1<!\
             --hk=_0-1-4c|leptos-dyn-child-end-->!</span><button \
             id=\"_0-1-5\">+1</button></div><!--leptos-view|leptos-tests-ssr.\
             rs-90|close--><!--hk=_0-1-0c|leptos-snake-case-counter-end--><!\
             --hk=_0-1-5-0o|leptos-snake-case-counter-start--><!\
             --leptos-view|leptos-tests-ssr.rs-90|open--><div \
             id=\"_0-1-5-1\"><button id=\"_0-1-5-2\">-1</button><span \
             id=\"_0-1-5-3\">Value: \
             <!--hk=_0-1-5-4o|leptos-dyn-child-start-->2<!\
             --hk=_0-1-5-4c|leptos-dyn-child-end-->!</span><button \
             id=\"_0-1-5-5\">+1</button></div><!\
             --leptos-view|leptos-tests-ssr.rs-90|close--><!\
             --hk=_0-1-5-0c|leptos-snake-case-counter-end--></div><!\
             --leptos-view|leptos-tests-ssr.rs-101|close-->"
        );
    });
}

#[cfg(not(any(feature = "csr", feature = "hydrate")))]
#[test]
fn test_classes() {
    use leptos::*;

    _ = create_scope(create_runtime(), |cx| {
        let (value, _set_value) = create_signal( 5);
        let rendered = view! {
            
            <div class="my big" class:a={move || value.get() > 10} class:red=true class:car={move || value.get() > 1}></div>
        };

        assert_eq!(
            rendered.into_view().render_to_string(),
            "<!--leptos-view|leptos-tests-ssr.rs-142|open--><div id=\"_0-1\" \
             class=\"my big  red \
             car\"></div><!--leptos-view|leptos-tests-ssr.rs-142|close-->"
        );
    });
}

#[cfg(not(any(feature = "csr", feature = "hydrate")))]
#[test]
fn ssr_with_styles() {
    use leptos::*;

    _ = create_scope(create_runtime(), |cx| {
        let (_, set_value) = create_signal( 0);
        let styles = "myclass";
        let rendered = view! {
             class = styles,
            <div>
                <button class="btn" on:click=move |_| set_value.update(|value| *value -= 1)>"-1"</button>
            </div>
        };

        assert_eq!(
            rendered.into_view().render_to_string(),
            "<!--leptos-view|leptos-tests-ssr.rs-164|open--><div id=\"_0-1\" \
             class=\" myclass\"><button id=\"_0-2\" class=\"btn \
             myclass\">-1</button></div><!--leptos-view|leptos-tests-ssr.\
             rs-164|close-->"
        );
    });
}

#[cfg(not(any(feature = "csr", feature = "hydrate")))]
#[test]
fn ssr_option() {
    use leptos::*;

    _ = create_scope(create_runtime(), |cx| {
        let (_, _) = create_signal( 0);
        let rendered = view! {
            
            <option/>
        };

        assert_eq!(
            rendered.into_view().render_to_string(),
            "<!--leptos-view|leptos-tests-ssr.rs-188|open--><option \
             id=\"_0-1\"></option><!--leptos-view|leptos-tests-ssr.\
             rs-188|close-->"
        );
    });
}
