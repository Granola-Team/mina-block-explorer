describe('navigation to block spotlight', () => {
    [{
        origin: '/summary/accounts/B62qqVBAmXHUJg8BxKM9J1XghP9yGRAfq6JCES6eDGCiHFWtK259m2q',
        selector: 'dialog a[href^="/accounts"]'
    },
    {
        origin: '/blocks/accounts/B62qrLewpuQqr1CTZP6SWuhvp9Me433b9q9u9Bz9jwZZ7ApsBGvSEU6',
        selector: 'dialog a[href^="/accounts"]'
    },
    {
        origin: '/accounts/B62qqVBAmXHUJg8BxKM9J1XghP9yGRAfq6JCES6eDGCiHFWtK259m2q',
        selector: 'a[href^="/accounts/"]'
    },
    {
        origin: '/transactions',
        selector: 'a[href^="/accounts/"]'
    }].forEach(({ origin, selector }) => it(`navigates to the block spotlight page from page ${origin}`,() => {
        cy.visit(origin);
        let link = cy.get(selector, {timeout: 10000}).first();
        link.then($a => {
            link.click({force:true});
            cy.url().should('include',$a.attr('href'), {timeout: 10000})
        })
    }));
});