get_active_button();
check_first_load();

on('body', 'click', '.a_1', function() {
  is = this;
  if (!is.classList.contains("active")){
    is.nextElementSibling.classList.remove("active");is.nextElementSibling.classList.add("pointer")
    is.classList.add("active"); is.classList.remove("pointer")
    nav = is.parentElement.parentElement;
    tabs = nav.nextElementSibling;
    tab_item = tabs.querySelector(".auth_tab_2");
    tab_item.classList.remove("active", "in");
    cur = tabs.querySelector(".auth_tab_1");
    cur.classList.add("active", "in")
}
});

on('body', 'click', '.a_2', function() {
  is = this;
  if (!is.classList.contains("active")){
    is.previousElementSibling.classList.remove("active");is.previousElementSibling.classList.add("pointer")
    is.classList.add("active"); is.classList.remove("pointer")
    nav = is.parentElement.parentElement;
    tabs = nav.nextElementSibling;
    tab_item = tabs.querySelector(".auth_tab_1");
    tab_item.classList.remove("active", "in");
    cur = tabs.querySelector(".auth_tab_2");
    cur.classList.add("active", "in")
}
});

on('body', 'click', '.apps_btn', function() {
  toggle_nav_first_span();
});
on('body', 'click', '.pages_btn', function() {
  toggle_nav_second_span();
});

on('body', 'click', '.window_fullscreen_hide', function() {
  mob_menu_hide()
});
on('body', 'click', '.mob_menu', function() {
  this.style.display = "none";
  document.querySelector(".window_fullscreen").style.display = "block";
});
