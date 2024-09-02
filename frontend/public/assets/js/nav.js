if (!window.X_NAV_EVENTS) {
  window.X_NAV_EVENTS = true;

  document.addEventListener('click', (e) => {
    if (e.target.closest('#main-menu-burger')) {
      toggleLogoutMenu();
    }
  });

  document.addEventListener('click', (e) => {
    if (e.target.closest('#btn-logout')) {
      toggleLogoutMenu();
    }
  });

  function handleLogout() {
    toggleLogoutMenu();
  }

  function toggleLogoutMenu() {
    const menu = document.getElementById('main-menu');
    if (menu) {
      menu.classList.toggle('is-active');
    }

    const burger = document.getElementById('main-menu-burger');
    if (burger) {
      burger.classList.toggle('is-active');
    }
  }
}
