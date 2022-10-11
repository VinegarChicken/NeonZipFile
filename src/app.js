const lib = require("../index.node");
const $ = require("jquery");


$("#OpenFile").on('click', function(){
    var files;
    try {
        files = lib.select_zip();
        for(i=0;i<files.length;i++){
            $("body").append(`<button class="file" id="${i}">${files[i]}</button>`)
            console.log(`${i} file: ${files[i]}`)
        }
    } catch (error) {
        alert(error);
    }
});

$("#ExtractAll").on('click', function(){
    lib.extract_all();
});

$(document).on('click', '.file', function(){
    lib.extract_zip(parseInt($(this).attr('id')))
});