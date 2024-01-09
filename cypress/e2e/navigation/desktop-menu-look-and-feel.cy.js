
const pages = require('../../pages.json');

describe('desktop', () => {
    
    it(`has menu in header`, () => {
        cy.visit("/summary");
        cy.get('header').matchImageSnapshot(`desktop-header`);
    });
    
});