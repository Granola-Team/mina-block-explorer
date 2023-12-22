const { get } = require("http");

describe('mobile menu look and feel', () => {
    it('opens and closes the mobile menu', () => {
        cy.viewport('iphone-xr');
        cy.visit('/');
        cy.get('header').screenshot();
        cy.openMobileMenu();
        cy.get('nav').screenshot();
    });
  })