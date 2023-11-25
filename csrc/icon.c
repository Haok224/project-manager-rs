#include <windows.h>

// 函数声明
BOOL SetWindowIcon(const char* title, const char* path);

// 设置窗口图标的函数
BOOL SetWindowIcon(const char* title, const char* path) {
    HWND hwnd = FindWindow(NULL, title);  // 根据标题查找窗口句柄

    if (hwnd == NULL) {
        return FALSE;  // 未找到指定标题的窗口
    }

    HICON hIcon = (HICON)LoadImage(NULL, path, IMAGE_ICON, 0, 0, LR_LOADFROMFILE);  // 从文件加载图标

    if (hIcon == NULL) {
        return FALSE;  // 加载图标失败
    }

    SendMessage(hwnd, WM_SETICON, ICON_SMALL, (LPARAM)hIcon);  // 设置窗口小图标
    SendMessage(hwnd, WM_SETICON, ICON_BIG, (LPARAM)hIcon);    // 设置窗口大图标

    return TRUE;
}