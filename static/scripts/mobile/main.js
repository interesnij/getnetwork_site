ajax = document.body.querySelector("#ajax");
get_active_button();

on('#ajax', 'click', '.a_1', function() {
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

on('#ajax', 'click', '.a_2', function() {
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

on('body', 'click', '#register_ajax', function() {
  form = document.querySelector("#signup");
  if (!form.querySelector(".r_username").value){
    form.querySelector(".r_username").style.border = "1px #FF0000 solid";
    toast_error("Придумайте логин!");
  } else if (!form.querySelector(".r_email").value){
    form.querySelector(".r_email").style.border = "1px #FF0000 solid";
    toast_error("Введите Вашу почту!")
  } else if (!form.querySelector(".password1").value){
    form.querySelector(".password1").style.border = "1px #FF0000 solid";
    toast_error("Пароль - обязательное поле!")
  } else if (!form.querySelector(".password2").value){
    form.querySelector(".password2").style.border = "1px #FF0000 solid";
    toast_error("Введите пароль еще раз!")
  }
  if (form.querySelector(".r_username").value){form.querySelector(".r_username").style.border = "rgba(0, 0, 0, 0.2)";}
  if (form.querySelector(".r_email").value){form.querySelector(".r_email").style.border = "rgba(0, 0, 0, 0.2)";}
  if (form.querySelector(".password1").value){form.querySelector(".password1").style.border = "rgba(0, 0, 0, 0.2)";}
  if (form.querySelector(".password2").value){form.querySelector(".password2").style.border = "rgba(0, 0, 0, 0.2)";}

  form_data = new FormData(form);
  reg_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  reg_link.open( 'POST', "/rest-auth/registration/", true );
  reg_link.onreadystatechange = function () {
  if ( reg_link.readyState == 4 && reg_link.status == 201 ) {
    if (window.location.href == "http://getnetwork.site/signup/"){window.location.href = "/";}
    else {window.location.href=window.location.href}
    }};
  reg_link.send(form_data);
})

on('body', 'click', '.window_fullscreen_hide', function() {
  mob_menu_hide()
});
on('body', 'click', '.mob_menu', function() {
  this.style.display = "none";
  document.querySelector(".window_fullscreen").style.display = "block";
});
