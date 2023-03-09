fn main()
{
	slint_build::compile("ui/diebutton.slint").unwrap();
	slint_build::compile("ui/window.slint").unwrap();
}
