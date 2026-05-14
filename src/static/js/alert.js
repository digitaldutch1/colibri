
// Fade alert messages after 5 seconds
document.addEventListener("DOMContentLoaded", function () {
    const alerts = document.querySelectorAll(".alert:not(.persistent-alert)");

    alerts.forEach(function (alert) {
        setTimeout(function () {
            alert.style.transition = "opacity 0.5s ease";
            alert.style.opacity = "0";

            setTimeout(function () {
                alert.remove();
            }, 500);
        // Start fade animation after 5 seconds   
        }, 5000);
    });
});