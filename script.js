var SHOW = "block";
var HIDE = "none";
var DESC_INPUT = "specialDesc";
var DESC_FLAG = "specialFlag";
//  const map = new Map<boolean, string>([[true, SHOW], [false, HIDE]]);


//// https://docs.rs/actix-web/latest/actix_web/cookie/struct.Cookie.html      COOKIE
function handleInputChange(value) {
    var element = document.getElementById(DESC_FLAG);
    var isChecked = element.checked;
    console.log('Вы ввели:' + isChecked);
    var descField = document.getElementById(DESC_INPUT);
    var setVisible = HIDE;
    if (isChecked) {
        setVisible = SHOW;
        descField.style.display = setVisible;
        return;
    }
    alert("clear up");
    descField.style.display = setVisible;
    descField.value = "";
    // Дополнительная логика
}
document.addEventListener('DOMContentLoaded', function () {
    var input = document.getElementById('specialFlag');
    input.addEventListener('input', function (e) {
        var target = e.target;
        handleInputChange(target.value);
    });
});
handleInputChange("");
