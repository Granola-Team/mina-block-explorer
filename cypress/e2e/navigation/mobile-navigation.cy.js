const pages = require('../../pages.json');

describe('mobile menu', () => {

    beforeEach(() => {
        cy.viewport('iphone-xr');
        cy.visit('/');
        cy.openMobileMenu();
    });

    pages.forEach(url => it(`provides navigation to ${url}`, () => { 
        cy.get(`nav a[href^="${url}"]`).click();
        cy.url().should('contain', url);
    }))
    
  })