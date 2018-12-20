use com_wrapper::ComWrapper;
use winapi::shared::guiddef::IID;
use winapi::um::unknwnbase::IUnknown;
use winapi::Interface;

pub unsafe trait SharedBitmapSource {
    fn get_shared_source(&self) -> (IID, &IUnknown);
}

unsafe impl SharedBitmapSource for dxgi::surface::Surface {
    fn get_shared_source(&self) -> (IID, &IUnknown) {
        unsafe { (<Self as ComWrapper>::Interface::uuidof(), &*self.get_raw()) }
    }
}
