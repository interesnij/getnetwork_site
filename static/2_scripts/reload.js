function loadScripts( src ) {
    var script = document.createElement("SCRIPT"),
        head = document.getElementsByTagName( "head" )[ 0 ],
        span = document.getElementsByTagName( "span" )[ 0 ],
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
    //span.appendChild( script );
}; 

function load_prev(ajax_link, elem_) {

}
function check_first_load() {
      url = window.location.href;
      ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'GET', url + "?ajax=1", true );
      ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            document.body.innerHTML = elem_.innerHTML;
            loadScripts('/static/2_scripts/custom.js?ver1');
            window.history.pushState ({"url":url}, document.title, url);
        }
      }
      ajax_link.send();

  };

  check_first_load();