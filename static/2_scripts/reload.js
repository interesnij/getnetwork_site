function loadScripts( src ) {
    var script = document.createElement("SCRIPT"),
        head = document.getElementsByTagName( "head" )[ 0 ],
        error = false;
    script.type = "text/javascript";
    script.onload = script.onreadystatechange = function( e ){
        if ( ( !this.readyState || this.readyState == "loaded" || this.readyState == "complete" ) ) {
            if ( !error ) {
                removeListeners();
            } else {
                null
            }
        }
    };
    script.onerror = function() {
        error = true;
        removeListeners();
    }
    function errorHandle( msg, url, line ) {
        if ( url == src ) {
            error = true;
            removeListeners();
        }
        return false;
    }
    function removeListeners() {
        script.onreadystatechange = script.onload = script.onerror = null;

        if ( window.removeEventListener ) {
            window.removeEventListener('error', errorHandle, false );
        } else {
            window.detachEvent("onerror", errorHandle );
        }
    }
    if ( window.addEventListener ) {
        window.addEventListener('error', errorHandle, false );
    } else {
        window.attachEvent("onerror", errorHandle );
    }
    script.src = src;
    head.appendChild( script );
};

function load_prev(ajax_link, elem_) {
    
}
function check_first_load() {
    span = document.body.querySelector(".span");
  
    if (window.location.href.indexOf('ajax=1') > -1) {
      span.innerHTML = "Permission Denied"; 
    }
    else if (!span.firstChild) {
      url = window.location.href;
      ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'GET', url + "?ajax=1", true );
      ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            //get_custom_design();
            
            //setTimeout(load_prev, 2000);
            loadScripts('/static/2_scripts/custom.js?ver1');
            loadScripts('/static/1_scripts/progressive-image.js');
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            span.innerHTML = elem_.innerHTML;
            //get_or_create_cookie_user(); 
            //get_active_button();
            //get_page_view_time(120);
            //scrolled(document.body.querySelector(".span"));
            window.history.pushState ({"url":url}, document.title, url);
        }
      }
      ajax_link.send();
    }
  };

  check_first_load();