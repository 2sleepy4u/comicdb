async function pageFunction(context) {    
    const extractValue = function(elem) {
        return $(elem).attr("content") || $(elem).text()
               || $(elem).attr("src") || $(elem).attr("href") || null;
    };
    const addProperty = function(item, propName, value) {
        if (typeof(value)==='string')
            value = value.trim();
        if (Array.isArray(item[propName]))
            item[propName].push(value);
        else if (typeof(item[propName])!=='undefined')
            item[propName] = [item[propName], value];
        else
            item[propName] = value;
    }
    const extractItem = function(elem) {
        let item = { _type: $(elem).attr("itemtype") };
        let count = 0;
        // iterate itemprops not nested in another itemscope    
        $(elem).find("[itemprop]").filter(function() {
            return $(this).parentsUntil(elem, '[itemscope]').length === 0;
        }).each( function() {
            addProperty(
                item,
                $(this).attr("itemprop"),
                $(this).is("[itemscope]")
                     ? extractItem(this)
                     : extractValue(this)
            );
            count++;
        });
        // special case - output at least something
        if( count===0 )
            addProperty(item, "_value", extractValue(elem));
        return item;
    };
    const extractAllItems = function() {
        const items = [];
        // find top-level itemscope elements
        $("[itemscope]").filter(function() {
            return $(this).parentsUntil("body", '[itemscope]').length === 0;
        }).each( function() {
            items.push( extractItem(this) );
        });

        return items;
    };    
    return extractAllItems();
}
async function copyToTheClipboard(textToCopy){
    const el = document.createElement('textarea');
    el.value = textToCopy;
    el.setAttribute('readonly', '');
    el.style.position = 'absolute';
    el.style.left = '-9999px';
    document.body.appendChild(el);
    el.select();
    document.execCommand('copy');
    document.body.removeChild(el);
}


