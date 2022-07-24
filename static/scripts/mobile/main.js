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
on('body', 'click', '#logg', function() {
  _this = this;
  form = _this.parentElement;
  response = form.querySelector(".api_response")
  if (!_user_phone){
    form.querySelector("#id_username").style.border = "1px #FF0000 solid";
    response.innerHTML = "Введите логин!";
    response.classList.add("error");
    return
  }
  else if (!form.querySelector("#id_password").value){
    form.querySelector("#id_password").style.border = "1px #FF0000 solid";
    response.innerHTML = "Введите пароль!";
    response.classList.add("error")
    return
  }
  else {
    _this.disabled = true;
  }

  form_data = new FormData(form);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/login/", true );
  //link.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    window.location.href = "/"
    }
  else {
    _this.disabled = false;
    response.style.display = "block";
    response.innerHTML = "Логин или пароль - неверный!";
    response.classList.add("error");
    form.querySelector("#id_username").style.display = "block";
    form.querySelector("#id_username").value = '';
    form.querySelector("#id_password").value = '';
  }};
  link.send(form_data);
});

on('body', 'click', '.window_fullscreen_hide', function() {
  mob_menu_hide()
});
on('body', 'click', '.mob_menu', function() {
  this.style.display = "none";
  document.querySelector(".window_fullscreen").style.display = "block";
});
