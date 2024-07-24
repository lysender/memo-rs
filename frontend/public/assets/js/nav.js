if (!window.X_NAV_EVENTS) {
  window.X_NAV_EVENTS = true;

  document.addEventListener('click', (e) => {
    if (e.target.closest('#main-menu-burger')) {
      handleToggleMenu();
    }
  });

  document.addEventListener('click', (e) => {
    if (e.target.closest('#btn-logout')) {
      handleToggleMenu();
    }
  });
}

function handleLogout() {
  handleToggleMenu();
}

function handleToggleMenu() {
  const menu = document.getElementById('main-menu');
  if (menu) {
    menu.classList.toggle('is-active');
  }

  const burger = document.getElementById('main-menu-burger');
  if (burger) {
    burger.classList.toggle('is-active');
  }
}
