
// Automatically remove alert messages
document.addEventListener("DOMContentLoaded", function () {
    const alerts = document.querySelectorAll(".alert:not(.persistent-alert)");
    alerts.forEach(function (alert) {
        setTimeout(function () {
            alert.style.transition =
                "opacity 0.5s ease";

            alert.style.opacity = "0";

            setTimeout(function () {
                const wrapper =
                    alert.closest(".alert-wrapper");

                if (wrapper) {
                    wrapper.remove();

                } else {
                    alert.remove();
                }

            }, 500);

        // Keep alert visible for 5 seconds before fading out
        }, 5000);
    });
});