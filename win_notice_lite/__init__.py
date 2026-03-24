from typing import (
    Callable,
    Union,
    Type,
    TypedDict,
    Optional
)

from win_notice_lite.win_notice_lite import *


__author__ = win_notice_lite.__author__
__license__ = win_notice_lite.__license__
__home__ = win_notice_lite.__home__
__description__ = win_notice_lite.__description__
__version__ = win_notice_lite.__version__


class ToastDict(TypedDict): # for typing
    id             : int
    tag            : Optional[str]
    name           : str
    title          : str
    group          : Optional[str]
    message        : str
    logo_uri       : Optional[str]
    fingerprint    : Optional[str]
    creation_time  : str
    inline_images  : list[str]
    hero_image_uri : Optional[str]
    fingerprint_without_time: Optional[str]


def __bind(
        target: type[Toast | MutableToast]
) -> Callable[[ToastDict], Toast]:
    def from_dict(
            source: ToastDict
    ) -> Toast | MutableToast:
        return target(
            id=source["id"],
            name=source["name"],
            logo_uri=source.get("logo_uri", ""),
            # use `get` to allow some empty keys
            title=source["title"],
            message=source["message"],
            hero_image_uri=source.get("hero_image_uri", ""),
            inline_images=source.get("inline_images", []),
            tag=source.get("tag", ""),
            group=source.get("group", ""),
            creation_time=source["creation_time"],
            fingerprint=source.get("fingerprint", ""),
            fingerprint_without_time=source.get("fingerprint_without_time", "")
        )
    def closure(source: ToastDict) -> Toast:
        return from_dict(source=source)
    return closure


Toast.from_dict = __bind(Toast)
MutableToast.from_dict = __bind(MutableToast)

__all__ = win_notice_lite.__all__

# clean up
del Callable, Union, Type, TypedDict, Optional
# noinspection PyUnresolvedReferences
del win_notice_lite
del __bind
