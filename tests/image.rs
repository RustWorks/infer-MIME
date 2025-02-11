mod common;

test_format!(Image, "image/jpeg", "jpg", jpg, "sample.jpg");

test_format!(Image, "image/png", "png", png, "sample.png");

test_format!(Image, "image/gif", "gif", gif, "sample.gif");

test_format!(Image, "image/tiff", "tif", tif, "sample.tif");

test_format!(Image, "image/tiff", "tif", tif2, "sample2.tif");

test_format!(Image, "image/tiff", "tif", tif3, "sample3.tif");

test_format!(Image, "image/tiff", "tif", tif4, "sample4.tif");

test_format!(Image, "image/tiff", "tif", tif5, "sample5.tif");

test_format!(Image, "image/bmp", "bmp", bmp, "sample.bmp");

test_format!(Image, "image/vnd.adobe.photoshop", "psd", psd, "sample.psd");

test_format!(Image, "image/vnd.microsoft.icon", "ico", ico, "sample.ico");

test_format!(Image, "image/heif", "heif", heif, "sample.heic");

test_format!(Image, "image/heif", "heif", heif2, "sample2.heic");

test_format!(Image, "image/avif", "avif", avif, "sample.avif");

test_format!(Image, "image/jxl", "jxl", jxl, "spline_on_first_frame.jxl");

test_format!(Image, "image/openraster", "ora", ora, "sample.ora");

test_format!(Image, "image/vnd.djvu", "djvu", djvu1, "sample_single.djvu");

test_format!(Image, "image/vnd.djvu", "djvu", djvu2, "sample_multi.djvu");
